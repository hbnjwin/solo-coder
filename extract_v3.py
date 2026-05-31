"""
种子清理卡 OCR v3 - 像素密度法
核心改进：用形态学去除印刷线条后的残余笔画密度来判断勾选状态，
替代 v2 的 OCR 文本前缀法。

检测原理：
- 勾选框的印刷边框是水平/垂直线条
- 手写勾选标记（✓/✗）包含斜向笔画
- 用形态学开运算去除水平和垂直线条后，剩余的就是手写笔画
- 通过残余笔画的像素占比判断是否被勾选

判断策略：
1. 计算每个选项的 stroke_score（残余笔画密度）
2. 同一字段内做相对比较：score > 字段中位数 * 2 且 > 0.025 → 已勾选
3. OCR 前缀作为辅助验证
"""

import json
from pathlib import Path

import cv2
import numpy as np
import easyocr


# ============================================================
# 透视矫正
# ============================================================

def correct_perspective(img: np.ndarray) -> np.ndarray:
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    blurred = cv2.GaussianBlur(gray, (5, 5), 0)
    edges = cv2.Canny(blurred, 50, 150)

    kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (3, 3))
    edges = cv2.dilate(edges, kernel, iterations=2)

    contours, _ = cv2.findContours(edges, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    if not contours:
        return img

    largest = max(contours, key=cv2.contourArea)
    area = cv2.contourArea(largest)
    img_area = img.shape[0] * img.shape[1]

    if area < img_area * 0.3:
        return img

    peri = cv2.arcLength(largest, True)
    approx = cv2.approxPolyDP(largest, 0.02 * peri, True)

    if len(approx) != 4:
        rect = cv2.minAreaRect(largest)
        angle = rect[2]
        if abs(angle) < 15 or abs(angle - 90) < 15:
            rot_angle = angle if abs(angle) < 15 else angle - 90
            h, w = img.shape[:2]
            center = (w // 2, h // 2)
            M = cv2.getRotationMatrix2D(center, rot_angle, 1.0)
            img = cv2.warpAffine(img, M, (w, h), flags=cv2.INTER_LINEAR,
                                 borderMode=cv2.BORDER_REPLICATE)
        return img

    pts = approx.reshape(4, 2).astype(np.float32)
    pts = _order_points(pts)

    w_top = np.linalg.norm(pts[1] - pts[0])
    w_bot = np.linalg.norm(pts[2] - pts[3])
    h_left = np.linalg.norm(pts[3] - pts[0])
    h_right = np.linalg.norm(pts[2] - pts[1])

    max_w = int(max(w_top, w_bot))
    max_h = int(max(h_left, h_right))

    dst = np.array([
        [0, 0], [max_w - 1, 0],
        [max_w - 1, max_h - 1], [0, max_h - 1]
    ], dtype=np.float32)

    M = cv2.getPerspectiveTransform(pts, dst)
    return cv2.warpPerspective(img, M, (max_w, max_h))


def _order_points(pts):
    rect = np.zeros((4, 2), dtype=np.float32)
    s = pts.sum(axis=1)
    rect[0] = pts[np.argmin(s)]
    rect[2] = pts[np.argmax(s)]
    d = np.diff(pts, axis=1)
    rect[1] = pts[np.argmin(d)]
    rect[3] = pts[np.argmax(d)]
    return rect


# ============================================================
# 配置
# ============================================================

UNCHECKED_PREFIX = "口"
CHECKED_PREFIXES = set("8巳0OGQ石曰5⑤☑■●◉✓√$2169SsBbDdPp&")

FIELD_LABELS = {"包装袋类型", "纯净度", "材料类型", "种子情况", "是否需要清理",
                "清理方法", "清理结果", "残渣量", "完成时间", "转历史",
                "异常情况", "X光取样", "可多选", "登记时间", "清理人",
                "取样人", "重清理"}

FUZZY_MAP = {
    "干果法": ["干果法", "午果法", "千果法", "于果法"],
    "干果": ["干果", "午果", "千果"],
    "已散": ["已散", "己散"],
}

ANCHOR1_LABEL = "包装袋类型"
ANCHOR1_BASE_Y = 548
ANCHOR2_LABEL = "完成时间"
ANCHOR2_BASE_Y = 1344
BASE_SPAN = ANCHOR2_BASE_Y - ANCHOR1_BASE_Y

FIELD_LAYOUT = [
    {
        "name": "registration_date",
        "name_cn": "登记时间",
        "type": "handwrite",
        "y_offset": (-120, -30),
        "x_range": (300, 650),
    },
    {
        "name": "cleaner_registration",
        "name_cn": "清理人（登记）",
        "type": "handwrite",
        "y_offset": (-120, -30),
        "x_range": (900, 1200),
    },
    {
        "name": "bag_type",
        "name_cn": "包装袋类型",
        "type": "radio",
        "y_offset": (-25, 50),
        "x_range": (300, 1200),
        "options": ["网袋", "布袋", "纸袋", "塑料袋", "其它"],
        "options_en": ["mesh_bag", "cloth_bag", "paper_bag", "plastic_bag", "other"],
    },
    {
        "name": "purity",
        "name_cn": "纯净度",
        "type": "checkbox_multi",
        "y_offset": (40, 130),
        "x_range": (300, 1200),
        "options": ["种子", "枝叶", "昆虫", "土沙", "其他种"],
        "options_full": ["种子", "枝叶", "昆虫(或其他病虫害)", "土沙砾", "其他种子"],
        "options_en": ["seeds", "branches_leaves", "insects", "soil_gravel", "other_seeds"],
    },
    {
        "name": "material_type",
        "name_cn": "材料类型",
        "type": "radio",
        "y_offset": (140, 210),
        "x_range": (300, 1200),
        "options": ["干果", "浆果", "清理过", "肉质", "净种子"],
        "options_full": ["干果", "浆果", "清理过的种子或果实", "肉质果实", "净种子"],
        "options_en": ["dry_fruit", "berry", "cleaned_seeds_or_fruit", "fleshy_fruit", "pure_seeds"],
    },
    {
        "name": "seed_condition",
        "name_cn": "种子情况",
        "type": "checkbox_multi",
        "y_offset": (200, 370),
        "x_range": (300, 1200),
        "options": ["完全成", "部分成", "未成熟", "已散", "虫蛀严", "空瘪严"],
        "options_full": ["完全成熟", "部分成熟", "未成熟", "已散布", "虫蛀严重", "空瘪严重"],
        "options_en": ["fully_mature", "partially_mature", "immature", "dispersed",
                       "severely_insect_damaged", "severely_shriveled"],
    },
    {
        "name": "needs_cleaning",
        "name_cn": "是否需要清理",
        "type": "radio",
        "y_offset": (290, 370),
        "x_range": (660, 1200),
        "options": ["是", "否"],
        "options_en": ["yes", "no"],
    },
    {
        "name": "cleaning_method",
        "name_cn": "清理方法",
        "type": "radio",
        "y_offset": (480, 560),
        "x_range": (300, 1200),
        "options": ["浆果法", "干果法", "肉质果法", "人工分拣"],
        "options_full": ["浆果法", "干果法", "肉质果法", "人工分拣杂质"],
        "options_en": ["berry_method", "dry_fruit_method", "fleshy_fruit_method", "manual_sorting"],
    },
    {
        "name": "result_type",
        "name_cn": "清理结果",
        "type": "radio",
        "y_offset": (560, 630),
        "x_range": (300, 1200),
        "options": ["种子", "果实"],
        "options_en": ["seeds", "fruit"],
    },
    {
        "name": "residue_amount",
        "name_cn": "残渣量",
        "type": "radio",
        "y_offset": (620, 690),
        "x_range": (300, 1200),
        "options": ["无", "极少", "少量"],
        "options_en": ["none", "very_little", "small_amount"],
    },
    {
        "name": "cleaner_completion",
        "name_cn": "清理人(完成)",
        "type": "handwrite",
        "y_offset": (680, 750),
        "x_range": (300, 650),
    },
    {
        "name": "sampler",
        "name_cn": "取样人",
        "type": "handwrite",
        "y_offset": (680, 750),
        "x_range": (850, 1080),
    },
    {
        "name": "completion_date",
        "name_cn": "完成时间",
        "type": "handwrite",
        "y_offset": (760, 840),
        "x_range": (300, 650),
    },
    {
        "name": "xray_sample_quantity",
        "name_cn": "X光取样数量",
        "type": "radio",
        "y_offset": (760, 840),
        "x_range": (850, 1200),
        "options": ["05", "10", "20", "30"],
        "options_full": ["5", "10", "20", "30"],
        "options_en": ["5", "10", "20", "30"],
    },
    {
        "name": "transfer_to_history",
        "name_cn": "转历史",
        "type": "radio",
        "y_offset": (830, 900),
        "x_range": (300, 650),
        "options": ["否", "是"],
        "options_en": ["no", "yes"],
    },
    {
        "name": "abnormal_conditions",
        "name_cn": "异常情况",
        "type": "checkbox_multi",
        "y_offset": (880, 1080),
        "x_range": (88, 1200),
        "options": ["种子已", "种子空瘪", "编号混乱", "无种子", "种子混杂",
                    "种子受损", "种子虫蛀", "其他原因"],
        "options_full": ["种子已发芽", "种子空瘪或不成熟", "编号混乱", "无种子",
                         "种子混杂", "种子受损严重", "种子虫蛀或霉变严重", "其他原因"],
        "options_en": ["seeds_germinated", "seeds_shriveled_or_immature", "numbering_disorder",
                       "no_seeds", "seeds_mixed", "seeds_severely_damaged",
                       "seeds_insect_or_mold_damaged", "other_reasons"],
    },
]


# ============================================================
# 核心检测器
# ============================================================

class SeedCardExtractorV3:
    def __init__(self, debug: bool = False):
        self.reader = easyocr.Reader(["ch_sim", "en"], gpu=False)
        self.debug = debug

    def process(self, image_path: str, output_dir: str = "crops") -> dict:
        img = cv2.imdecode(np.fromfile(image_path, dtype=np.uint8), cv2.IMREAD_COLOR)
        if img is None:
            return {"error": f"无法读取: {image_path}"}

        img = correct_perspective(img)
        img_h, img_w = img.shape[:2]
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

        ocr_results = self.reader.readtext(img)
        blocks = self._parse_blocks(ocr_results)
        anchor1_y, scale = self._find_anchors(blocks)

        result = {}
        crop_dir = Path(output_dir)
        crop_dir.mkdir(parents=True, exist_ok=True)

        for field in FIELD_LAYOUT:
            y1 = int(anchor1_y + field["y_offset"][0] * scale)
            y2 = int(anchor1_y + field["y_offset"][1] * scale)
            x1, x2 = field["x_range"]

            y1 = max(0, min(y1, img_h - 1))
            y2 = max(0, min(y2, img_h - 1))
            x1 = max(0, min(x1, img_w - 1))
            x2 = max(0, min(x2, img_w - 1))

            if field["type"] == "handwrite":
                crop = img[y1:y2, x1:x2]
                if crop.size > 0:
                    crop_path = crop_dir / f"{field['name']}.jpg"
                    cv2.imencode('.jpg', crop)[1].tofile(str(crop_path))
                region_blocks = [b for b in blocks if y1 <= b["yc"] <= y2 and x1 <= b["xc"] <= x2]
                result[field["name"]] = self._extract_handwrite(region_blocks)

            elif field["type"] in ("radio", "checkbox_multi"):
                region_blocks = [b for b in blocks if y1 <= b["yc"] <= y2 and x1 <= b["xc"] <= x2]
                checked = self._detect_checked_by_density(gray, region_blocks, field)

                if field["type"] == "checkbox_multi":
                    result[field["name"]] = checked
                else:
                    result[field["name"]] = checked[0] if checked else None

                crop = img[y1:y2, x1:x2]
                if crop.size > 0:
                    crop_path = crop_dir / f"{field['name']}.jpg"
                    cv2.imencode('.jpg', crop)[1].tofile(str(crop_path))

        return result

    # ----------------------------------------------------------
    # 像素密度法：去除印刷线条后检测手写笔画
    # ----------------------------------------------------------

    def _compute_stroke_score(self, gray: np.ndarray, x1: int, y1: int, x2: int, y2: int,
                              cb_width: int = 32) -> float:
        """
        计算 checkbox 区域的手写笔画密度。
        从 OCR bbox 的左侧提取 checkbox 区域，去除水平/垂直印刷线条，
        测量剩余的斜向笔画占比。
        """
        roi = gray[y1:y2, x1:x1 + cb_width]
        if roi.size == 0 or roi.shape[0] < 8 or roi.shape[1] < 8:
            return 0.0

        _, binary = cv2.threshold(roi, 0, 255, cv2.THRESH_BINARY_INV + cv2.THRESH_OTSU)

        # 去除水平线条（印刷的横线/框线）
        h_kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (12, 1))
        h_lines = cv2.morphologyEx(binary, cv2.MORPH_OPEN, h_kernel)

        # 去除垂直线条（印刷的竖线/框线）
        v_kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (1, 12))
        v_lines = cv2.morphologyEx(binary, cv2.MORPH_OPEN, v_kernel)

        # 合并所有印刷线条
        hv_mask = cv2.bitwise_or(h_lines, v_lines)

        # 从二值图中减去印刷线条，剩余的就是手写笔画
        remaining = cv2.subtract(binary, hv_mask)

        return float(np.sum(remaining > 0) / remaining.size)

    def _detect_checked_by_density(self, gray: np.ndarray, blocks: list[dict],
                                   field: dict) -> list[str]:
        """
        混合检测策略：
        1. 对每个选项计算 stroke_score（像素密度）
        2. 同一字段内做相对比较
        3. OCR 前缀作为辅助信号
        """
        options = field["options"]
        options_en = field.get("options_en", options)

        # 为每个选项收集信息
        option_data = []
        for i, opt_key in enumerate(options):
            block = self._find_option_block(blocks, opt_key)
            if block is None:
                option_data.append({
                    "index": i, "block": None,
                    "stroke_score": 0.0, "ocr_checked": False
                })
                continue

            # 计算像素密度
            stroke_score = self._compute_stroke_score(
                gray, block["x1"], block["y1"], block["x2"], block["y2"]
            )

            # OCR 前缀判断（作为辅助信号）
            ocr_checked = self._ocr_prefix_check(block["text"], opt_key)

            option_data.append({
                "index": i, "block": block,
                "stroke_score": stroke_score, "ocr_checked": ocr_checked
            })

        # 决策：结合密度和 OCR 前缀
        checked = self._make_decision(option_data, field)

        return [options_en[d["index"]] for d in checked]

    def _make_decision(self, option_data: list[dict], field: dict) -> list[dict]:
        """
        决策逻辑：
        - 高密度 (> 0.04) + OCR确认 → 确定已勾选
        - 高密度 (> 0.04) + OCR不确认 → 可能已勾选（密度优先）
        - 低密度 (< 0.015) → 确定未勾选
        - 中间地带 → 用相对比较 + OCR 辅助决定
        """
        scores = [d["stroke_score"] for d in option_data if d["block"] is not None]
        if not scores:
            # 没有找到任何选项块，回退到纯 OCR
            return [d for d in option_data if d["ocr_checked"]]

        median_score = float(np.median(scores))
        max_score = max(scores)

        checked = []
        for d in option_data:
            if d["block"] is None:
                continue

            score = d["stroke_score"]
            ocr = d["ocr_checked"]

            # 策略1：绝对高密度 → 已勾选
            if score > 0.045:
                checked.append(d)
                continue

            # 策略2：绝对低密度 → 未勾选
            if score < 0.012:
                continue

            # 策略3：相对比较 - 显著高于中位数
            if score > median_score * 2.5 and score > 0.025:
                checked.append(d)
                continue

            # 策略4：密度适中 + OCR 确认
            if score > 0.02 and ocr:
                checked.append(d)
                continue

            # 策略5：OCR 强信号（密度不够但 OCR 明确）
            # 仅当密度不是特别低时才信任 OCR
            if ocr and score > 0.015:
                checked.append(d)
                continue

        # 对 radio 类型，如果检测到多个，取密度最高的
        if field["type"] == "radio" and len(checked) > 1:
            checked.sort(key=lambda d: d["stroke_score"], reverse=True)
            checked = [checked[0]]

        if self.debug:
            print(f"  [{field['name']}] median={median_score:.4f} max={max_score:.4f}")
            for d in option_data:
                marker = "*" if d in checked else " "
                opt_name = field["options"][d["index"]]
                print(f"    {marker} {opt_name} score={d['stroke_score']:.4f} ocr={d['ocr_checked']}")

        return checked

    # ----------------------------------------------------------
    # OCR 前缀检测（辅助方法，保留作为备用信号）
    # ----------------------------------------------------------

    def _ocr_prefix_check(self, text: str, opt_key: str) -> bool:
        idx = text.find(opt_key)
        if idx < 0:
            for variants in FUZZY_MAP.values():
                for v in variants:
                    if v in text:
                        idx = text.find(v)
                        break
                if idx >= 0:
                    break
        if idx < 0:
            return False
        if idx == 0:
            return True
        prefix = text[:idx].strip()
        if not prefix:
            return True
        if UNCHECKED_PREFIX in prefix:
            return False
        if any(c in CHECKED_PREFIXES for c in prefix):
            return True
        if len(prefix) <= 2 and not any('一' <= c <= '鿿' for c in prefix):
            return True
        return False

    # ----------------------------------------------------------
    # 辅助方法
    # ----------------------------------------------------------

    def _parse_blocks(self, ocr_results) -> list[dict]:
        blocks = []
        for bbox, text, conf in ocr_results:
            xs = [p[0] for p in bbox]
            ys = [p[1] for p in bbox]
            blocks.append({
                "text": text.strip(),
                "xc": (min(xs) + max(xs)) / 2,
                "yc": (min(ys) + max(ys)) / 2,
                "x1": int(min(xs)), "y1": int(min(ys)),
                "x2": int(max(xs)), "y2": int(max(ys)),
                "conf": conf,
            })
        return blocks

    def _find_anchors(self, blocks: list[dict]) -> tuple[float, float]:
        anchor1_y = None
        anchor2_y = None
        for b in blocks:
            if "包装袋" in b["text"] or "袋类型" in b["text"]:
                anchor1_y = b["yc"]
            if "完成时" in b["text"] and b["xc"] < 400:
                anchor2_y = b["yc"]
        if anchor1_y is None:
            anchor1_y = ANCHOR1_BASE_Y
        if anchor2_y is not None:
            scale = (anchor2_y - anchor1_y) / BASE_SPAN
        else:
            scale = 1.0
        return anchor1_y, scale

    def _find_option_block(self, blocks: list[dict], opt_key: str) -> dict | None:
        candidates = []
        for b in blocks:
            text = b["text"]
            if any(lbl in text for lbl in FIELD_LABELS):
                continue
            if opt_key in text:
                candidates.append(b)

        if candidates:
            candidates.sort(key=lambda b: len(b["text"]))
            return candidates[0]

        fuzzy_variants = FUZZY_MAP.get(opt_key, [])
        for variant in fuzzy_variants:
            for b in blocks:
                if any(lbl in b["text"] for lbl in FIELD_LABELS):
                    continue
                if variant in b["text"]:
                    return b
        return None

    def _extract_handwrite(self, blocks: list[dict]) -> str:
        labels = ["登记时间", "昼记时间", "昼记", "清理人", "完成时间", "取样人", "X光",
                  "(完成)", "(登记)", "光取样", "包装袋", "纯净度", "材料类型", "种子情况",
                  "是否需要", "清理方法", "清理结果", "残渣量", "转历史", "异常",
                  "猜理", "淆理", "[瞿)", "(可多选)", "多选)"]
        numeric_options = {"05", "010", "020", "030", "10", "20", "30"}
        texts = []
        for b in blocks:
            t = b["text"].strip()
            if not t or t == "口":
                continue
            if any(lbl in t for lbl in labels):
                continue
            if t in numeric_options:
                continue
            if t.startswith("口"):
                continue
            texts.append(t)
        return " ".join(texts) if texts else ""


# ============================================================
# 入口
# ============================================================

def main():
    images = [
        r"d:\work\github\ocr\微信图片_20260519194813_10_879.jpg",
        r"d:\work\github\ocr\微信图片_20260519194831_11_879.jpg",
    ]

    extractor = SeedCardExtractorV3(debug=True)
    all_results = []

    for img_path in images:
        name = Path(img_path).stem
        print(f"\n{'='*60}")
        print(f"Processing: {Path(img_path).name}")
        print(f"{'='*60}")
        crop_dir = f"crops/{name}"
        result = extractor.process(img_path, output_dir=crop_dir)
        all_results.append({"file": Path(img_path).name, "extracted": result})
        print(f"\nResult:")
        for k, v in result.items():
            if v:
                print(f"  {k}: {v}")

    output = {"form_name": "seed_cleaning_card", "results": all_results}
    Path("output_v3.json").write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"\n\nSaved to output_v3.json")


if __name__ == "__main__":
    main()
