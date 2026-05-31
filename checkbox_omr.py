"""
种子清理卡 - 勾选框检测模块

检测方法：
1. 固定模板坐标（相对于锚点）定义每个 checkbox 位置
2. ORB 特征匹配计算单应性矩阵，处理透视变形
3. 去除印刷线条后的残余笔画密度判断勾选状态
4. 同字段内相对比较决策
"""

import json
from pathlib import Path

import cv2
import numpy as np
import easyocr


# ============================================================
# 图像对齐
# ============================================================

def deskew(img: np.ndarray) -> np.ndarray:
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY) if len(img.shape) == 3 else img
    edges = cv2.Canny(gray, 50, 150)
    lines = cv2.HoughLinesP(edges, 1, np.pi/180, threshold=100,
                            minLineLength=200, maxLineGap=10)
    if lines is None:
        return img

    angles = []
    for line in lines:
        x1, y1, x2, y2 = line[0]
        if abs(x2 - x1) > 50:
            angle = np.degrees(np.arctan2(y2 - y1, x2 - x1))
            if abs(angle) < 15:
                angles.append(angle)

    if not angles:
        return img

    median_angle = float(np.median(angles))
    if abs(median_angle) < 0.1:
        return img

    h, w = img.shape[:2]
    M = cv2.getRotationMatrix2D((w / 2, h / 2), median_angle, 1.0)
    return cv2.warpAffine(img, M, (w, h), flags=cv2.INTER_LINEAR,
                          borderMode=cv2.BORDER_REPLICATE)


def compute_homography(ref_gray: np.ndarray, target_gray: np.ndarray) -> np.ndarray | None:
    orb = cv2.ORB_create(nfeatures=5000)
    kp1, des1 = orb.detectAndCompute(ref_gray, None)
    kp2, des2 = orb.detectAndCompute(target_gray, None)

    if des1 is None or des2 is None:
        return None

    bf = cv2.BFMatcher(cv2.NORM_HAMMING, crossCheck=True)
    matches = sorted(bf.match(des1, des2), key=lambda x: x.distance)

    if len(matches) < 20:
        return None

    n = min(100, len(matches))
    pts1 = np.float32([kp1[m.queryIdx].pt for m in matches[:n]])
    pts2 = np.float32([kp2[m.trainIdx].pt for m in matches[:n]])

    H, mask = cv2.findHomography(pts1, pts2, cv2.RANSAC, 5.0)
    if H is None or mask.ravel().sum() < 15:
        return None

    return H


# ============================================================
# Checkbox 坐标模板（从参考图精确测量，相对于锚点）
# ============================================================

ANCHOR_BASE_Y = 548
ANCHOR2_LABEL_Y = 1344
BASE_SPAN = ANCHOR2_LABEL_Y - ANCHOR_BASE_Y  # 796

CHECKBOX_MAP = {
    "bag_type": {
        "type": "radio",
        "options": ["mesh_bag", "cloth_bag", "paper_bag", "plastic_bag", "other"],
        "options_cn": ["网袋", "布袋", "纸袋", "塑料袋", "其它"],
        "positions": [(370, 2), (526, 2), (690, 4), (910, -15), (1044, 5)],
    },
    "purity": {
        "type": "checkbox_multi",
        "options": ["seeds", "branches_leaves", "insects", "soil_gravel", "other_seeds"],
        "options_cn": ["种子", "枝叶", "昆虫(或其他病虫害)", "土沙砾", "其他种子"],
        "positions": [(370, 84), (526, 86), (690, 67), (910, 67), (1044, 66)],
    },
    "material_type": {
        "type": "radio",
        "options": ["dry_fruit", "berry", "cleaned_seeds_or_fruit", "fleshy_fruit", "pure_seeds"],
        "options_cn": ["干果", "浆果", "清理过的种子或果实", "肉质果实", "净种子"],
        "positions": [(370, 168), (526, 168), (690, 148), (910, 148), (1044, 169)],
    },
    "seed_condition": {
        "type": "checkbox_multi",
        "options": ["fully_mature", "partially_mature", "immature",
                    "dispersed", "severely_insect_damaged", "severely_shriveled"],
        "options_cn": ["完全成熟", "部分成熟", "未成熟", "已散布", "虫蛀严重", "空瘪严重"],
        "positions": [(370, 230), (526, 231), (690, 251),
                      (370, 314), (690, 232), (1044, 232)],
    },
    "needs_cleaning": {
        "type": "radio",
        "options": ["yes", "no"],
        "options_cn": ["是", "否"],
        "positions": [(910, 331), (1044, 335)],
    },
    "cleaning_method": {
        "type": "radio",
        "options": ["berry_method", "dry_fruit_method", "fleshy_fruit_method", "manual_sorting"],
        "options_cn": ["浆果法", "干果法", "肉质果法", "人工分拣杂质"],
        "positions": [(370, 519), (526, 519), (690, 517), (910, 518)],
    },
    "result_type": {
        "type": "radio",
        "options": ["seeds", "fruit"],
        "options_cn": ["种子", "果实"],
        "positions": [(370, 593), (526, 593)],
    },
    "residue_amount": {
        "type": "radio",
        "options": ["none", "very_little", "small_amount"],
        "options_cn": ["无", "极少", "少量"],
        "positions": [(370, 650), (526, 646), (690, 650)],
    },
    "xray_sample_quantity": {
        "type": "radio",
        "options": ["5", "10", "20", "30"],
        "options_cn": ["5", "10", "20", "30"],
        "positions": [(910, 793), (1044, 792), (910, 856), (1044, 856)],
    },
    "transfer_to_history": {
        "type": "radio",
        "options": ["no", "yes"],
        "options_cn": ["否", "是"],
        "positions": [(370, 848), (526, 848)],
    },
    "abnormal_conditions": {
        "type": "checkbox_multi",
        "options": ["numbering_disorder", "seeds_germinated", "no_seeds", "seeds_mixed",
                    "seeds_shriveled_or_immature", "seeds_severely_damaged",
                    "seeds_insect_or_mold_damaged", "other_reasons"],
        "options_cn": ["编号混乱", "种子已发芽", "无种子", "种子混杂",
                       "种子空瘪或不成熟", "种子受损严重", "种子虫蛀或霉变严重", "其他原因"],
        "positions": [(150, 902), (370, 902), (526, 902), (690, 902),
                      (150, 960), (526, 960), (690, 960), (910, 960)],
    },
}


# ============================================================
# 核心检测逻辑
# ============================================================

MEAN_THRESHOLD = 165


def compute_stroke_score(gray: np.ndarray, cx: int, cy: int,
                         threshold: int = 170, size: int = 25) -> float:
    half = size // 2
    border = 4

    h, w = gray.shape[:2]
    x1 = max(0, cx - half + border)
    y1 = max(0, cy - half + border)
    x2 = min(w, cx + half - border)
    y2 = min(h, cy + half - border)

    interior = gray[y1:y2, x1:x2]
    if interior.size == 0 or interior.shape[0] < 6 or interior.shape[1] < 6:
        return 0.0

    if float(interior.mean()) < MEAN_THRESHOLD:
        return 0.0

    _, binary = cv2.threshold(interior, threshold, 255, cv2.THRESH_BINARY_INV)

    ih, iw = binary.shape
    h_len = max(iw - 2, 5)
    v_len = max(ih - 2, 5)

    h_kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (h_len, 1))
    h_lines = cv2.morphologyEx(binary, cv2.MORPH_OPEN, h_kernel)

    v_kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (1, v_len))
    v_lines = cv2.morphologyEx(binary, cv2.MORPH_OPEN, v_kernel)

    hv_mask = cv2.bitwise_or(h_lines, v_lines)
    remaining = cv2.subtract(binary, hv_mask)

    return float(np.sum(remaining > 0) / remaining.size)


def detect_checkboxes(gray: np.ndarray, anchor_y: float, scale: float = 1.0,
                      homography: np.ndarray = None, debug: bool = False) -> dict:
    results = {}

    for field_name, field_def in CHECKBOX_MAP.items():
        positions = field_def["positions"]
        options = field_def["options"]
        field_type = field_def["type"]

        scores = []
        debug_positions = []

        for i, (cx, y_off) in enumerate(positions):
            cy = int(anchor_y + y_off * scale)

            if homography is not None:
                pt = np.array([[[float(cx), float(cy)]]], dtype=np.float32)
                transformed = cv2.perspectiveTransform(pt, homography)
                tx, ty = int(transformed[0][0][0]), int(transformed[0][0][1])
            else:
                tx, ty = cx, cy

            score = compute_stroke_score(gray, tx, ty)
            scores.append(score)
            debug_positions.append((tx, ty))

        checked_indices = _decide(scores, field_type)

        if debug:
            print(f"  {field_name}:")
            for i, (score, (tx, ty)) in enumerate(zip(scores, debug_positions)):
                marker = "*" if i in checked_indices else " "
                opt_cn = field_def["options_cn"][i] if i < len(field_def["options_cn"]) else "?"
                print(f"    {marker} {opt_cn:12s} score={score:.4f} pos=({tx},{ty})")

        if field_type == "radio":
            results[field_name] = options[checked_indices[0]] if checked_indices else None
        else:
            results[field_name] = [options[i] for i in checked_indices]

    return results


def _decide(scores: list[float], field_type: str) -> list[int]:
    if not scores or max(scores) < 0.015:
        return []

    arr = np.array(scores)
    sorted_scores = sorted(arr, reverse=True)
    max_score = sorted_scores[0]
    second_score = sorted_scores[1] if len(sorted_scores) > 1 else 0.0

    if field_type == "radio":
        if max_score < 0.018:
            return []
        if second_score > 0 and max_score / second_score < 2.0:
            return []
        best = int(np.argmax(arr))
        return [best]
    else:
        baseline = float(np.median(np.sort(arr)[:max(1, len(arr) - 1)]))
        checked = []
        for i, score in enumerate(scores):
            if score > 0.035 and (baseline == 0 or score > baseline * 3):
                checked.append(i)
        return checked


# ============================================================
# 完整流程
# ============================================================

class SeedCardOMR:
    """种子清理卡 OMR 检测器"""

    def __init__(self, reference_path: str = None, debug: bool = False):
        self.reader = easyocr.Reader(["ch_sim", "en"], gpu=False)
        self.debug = debug
        self.ref_gray = None
        self.ref_features = None

        if reference_path:
            ref_img = cv2.imdecode(np.fromfile(reference_path, dtype=np.uint8), cv2.IMREAD_COLOR)
            if ref_img is not None:
                self.ref_gray = cv2.cvtColor(ref_img, cv2.COLOR_BGR2GRAY)

    def process(self, image_path: str) -> dict:
        img = cv2.imdecode(np.fromfile(image_path, dtype=np.uint8), cv2.IMREAD_COLOR)
        if img is None:
            return {"error": f"Cannot read: {image_path}"}

        img = deskew(img)
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

        anchor_y, scale = self._find_anchors(gray)

        if self.debug:
            print(f"  Anchor Y={anchor_y:.0f}, Scale={scale:.3f}")

        homography = None
        if abs(scale - 1.0) > 0.05 and self.ref_gray is not None:
            homography = compute_homography(self.ref_gray, gray)
            if homography is not None and self.debug:
                print(f"  Using homography alignment")

        if homography is not None:
            checkbox_results = detect_checkboxes(gray, ANCHOR_BASE_Y, 1.0,
                                                homography=homography, debug=self.debug)
        else:
            checkbox_results = detect_checkboxes(gray, anchor_y, scale,
                                                debug=self.debug)

        return checkbox_results

    def _find_anchors(self, gray: np.ndarray) -> tuple[float, float]:
        img_bgr = cv2.cvtColor(gray, cv2.COLOR_GRAY2BGR)
        ocr_results = self.reader.readtext(img_bgr)

        anchor1_y = None
        anchor2_y = None

        for bbox, text, conf in ocr_results:
            ys = [p[1] for p in bbox]
            xs = [p[0] for p in bbox]
            yc = (min(ys) + max(ys)) / 2
            xc = (min(xs) + max(xs)) / 2

            if "包装袋" in text or "袋类型" in text:
                anchor1_y = yc
            if ("完成时" in text) and xc < 400:
                anchor2_y = yc

        if anchor1_y is None:
            anchor1_y = ANCHOR_BASE_Y
        if anchor2_y is not None:
            scale = (anchor2_y - anchor1_y) / BASE_SPAN
        else:
            scale = 1.0

        return anchor1_y, scale


# ============================================================
# 入口
# ============================================================

def main():
    images = [
        r"d:\work\github\ocr\微信图片_20260519194813_10_879.jpg",
        r"d:\work\github\ocr\微信图片_20260519194831_11_879.jpg",
    ]

    ground_truth = {
        "bag_type": "mesh_bag",
        "purity": ["seeds"],
        "material_type": None,
        "seed_condition": ["fully_mature"],
        "needs_cleaning": "yes",
        "cleaning_method": "dry_fruit_method",
        "result_type": "seeds",
        "residue_amount": "very_little",
        "xray_sample_quantity": "30",
        "transfer_to_history": None,
        "abnormal_conditions": [],
    }

    # Use image 1 as reference for alignment
    detector = SeedCardOMR(reference_path=images[0], debug=True)

    for img_path in images:
        print(f"\n{'='*60}")
        print(f"Processing: {Path(img_path).name}")
        print(f"{'='*60}")

        result = detector.process(img_path)

        print(f"\n  Results vs Ground Truth:")
        correct = 0
        total = 0
        for field, gt_value in ground_truth.items():
            detected = result.get(field)
            match = detected == gt_value
            correct += int(match)
            total += 1
            marker = "OK" if match else "WRONG"
            print(f"    {marker:5s} {field:25s} detected={str(detected):30s} expected={str(gt_value)}")

        print(f"\n  Accuracy: {correct}/{total} ({correct/total*100:.0f}%)")

    output = {"form_name": "seed_cleaning_card", "results": []}
    for img_path in images:
        result = SeedCardOMR(reference_path=images[0], debug=False).process(img_path)
        output["results"].append({"file": Path(img_path).name, "checkboxes": result})

    Path("output_omr.json").write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"\nSaved to output_omr.json")


if __name__ == "__main__":
    main()
