"""
Checkbox CNN 检测脚本

使用训练好的 CNN 模型检测 checkbox 勾选状态。
替代原有的像素密度/阈值判断逻辑，提供更鲁棒的分类。

用法:
    python checkbox_cnn_detect.py [image_path]

依赖:
    models/checkbox_cnn.pth  - 训练好的模型（由 checkbox_cnn_train.py 生成）
"""

import json
import sys
from pathlib import Path

import cv2
import numpy as np
import torch
import torch.nn as nn


# ============================================================
# CNN 模型定义（与训练脚本一致）
# ============================================================

class CheckboxCNN(nn.Module):
    def __init__(self):
        super().__init__()
        self.features = nn.Sequential(
            nn.Conv2d(1, 16, 3, padding=1),
            nn.BatchNorm2d(16),
            nn.ReLU(inplace=True),
            nn.MaxPool2d(2),

            nn.Conv2d(16, 32, 3, padding=1),
            nn.BatchNorm2d(32),
            nn.ReLU(inplace=True),
            nn.MaxPool2d(2),

            nn.Conv2d(32, 64, 3, padding=1),
            nn.BatchNorm2d(64),
            nn.ReLU(inplace=True),
            nn.AdaptiveAvgPool2d(4),
        )
        self.classifier = nn.Sequential(
            nn.Dropout(0.3),
            nn.Linear(64 * 4 * 4, 64),
            nn.ReLU(inplace=True),
            nn.Dropout(0.3),
            nn.Linear(64, 2),
        )

    def forward(self, x):
        x = self.features(x)
        x = x.view(x.size(0), -1)
        x = self.classifier(x)
        return x


# ============================================================
# 图像对齐（复用核心逻辑）
# ============================================================

ANCHOR_BASE_Y = 548
ANCHOR2_LABEL_Y = 1344
BASE_SPAN = ANCHOR2_LABEL_Y - ANCHOR_BASE_Y

ANCHOR1_REGION = (155, 285, 530, 566)
ANCHOR2_REGION = (80, 200, 1322, 1360)

MATCH_SCALES = [0.85, 0.90, 0.95, 1.00, 1.05, 1.10, 1.15]
MATCH_THRESHOLD = 0.55

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


def deskew(img: np.ndarray) -> np.ndarray:
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY) if len(img.shape) == 3 else img
    edges = cv2.Canny(gray, 50, 150)
    lines = cv2.HoughLinesP(edges, 1, np.pi / 180, threshold=100,
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


def _match_template_multi_scale(gray, tmpl, scales=None, threshold=MATCH_THRESHOLD):
    if scales is None:
        scales = MATCH_SCALES
    best_val = 0.0
    best_loc = None
    best_scale = 1.0

    for s in scales:
        th, tw = tmpl.shape[:2]
        new_w = max(1, int(tw * s))
        new_h = max(1, int(th * s))
        scaled_tmpl = cv2.resize(tmpl, (new_w, new_h), interpolation=cv2.INTER_AREA)
        if scaled_tmpl.shape[0] > gray.shape[0] or scaled_tmpl.shape[1] > gray.shape[1]:
            continue
        result = cv2.matchTemplate(gray, scaled_tmpl, cv2.TM_CCOEFF_NORMED)
        _, max_val, _, max_loc = cv2.minMaxLoc(result)
        if max_val > best_val:
            best_val = max_val
            best_loc = max_loc
            best_scale = s

    if best_val < threshold or best_loc is None:
        return None

    th, tw = tmpl.shape[:2]
    cx = best_loc[0] + int(tw * best_scale / 2)
    cy = best_loc[1] + int(th * best_scale / 2)
    return float(cx), float(cy)


def crop_checkbox(gray: np.ndarray, cx: int, cy: int, size: int = 32) -> np.ndarray | None:
    half = size // 2
    h, w = gray.shape
    x1 = cx - half
    y1 = cy - half
    x2 = cx + half
    y2 = cy + half

    if x1 < 0 or y1 < 0 or x2 > w or y2 > h:
        pad_x1 = max(0, -x1)
        pad_y1 = max(0, -y1)
        pad_x2 = max(0, x2 - w)
        pad_y2 = max(0, y2 - h)
        x1 = max(0, x1)
        y1 = max(0, y1)
        x2 = min(w, x2)
        y2 = min(h, y2)
        crop = gray[y1:y2, x1:x2]
        crop = cv2.copyMakeBorder(crop, pad_y1, pad_y2, pad_x1, pad_x2,
                                  cv2.BORDER_REPLICATE)
    else:
        crop = gray[y1:y2, x1:x2]

    if crop.shape[0] != size or crop.shape[1] != size:
        crop = cv2.resize(crop, (size, size), interpolation=cv2.INTER_AREA)

    return crop


# ============================================================
# CNN 检测器
# ============================================================

class CheckboxCNNDetector:
    """使用 CNN 模型检测 checkbox 勾选状态"""

    def __init__(self, model_path: str, reference_path: str, debug: bool = False):
        self.debug = debug
        self.device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

        # 加载模型
        self.model = CheckboxCNN().to(self.device)
        checkpoint = torch.load(model_path, map_location=self.device, weights_only=True)
        self.model.load_state_dict(checkpoint['model_state_dict'])
        self.model.eval()

        if debug:
            print(f"  Model loaded from {model_path}")
            print(f"  Training accuracy: {checkpoint.get('accuracy', 'N/A')}")

        # 加载参考图模板
        ref_img = cv2.imdecode(np.fromfile(reference_path, dtype=np.uint8), cv2.IMREAD_COLOR)
        self.ref_gray = cv2.cvtColor(ref_img, cv2.COLOR_BGR2GRAY)

        x1, x2, y1, y2 = ANCHOR1_REGION
        self.tmpl_anchor1 = self.ref_gray[y1:y2, x1:x2].copy()
        x1, x2, y1, y2 = ANCHOR2_REGION
        self.tmpl_anchor2 = self.ref_gray[y1:y2, x1:x2].copy()

    def predict_crop(self, crop: np.ndarray) -> tuple[int, float]:
        """对单个 crop 预测: 返回 (label, confidence)"""
        tensor = torch.from_numpy(crop).float().unsqueeze(0).unsqueeze(0) / 255.0
        tensor = tensor.to(self.device)

        with torch.no_grad():
            output = self.model(tensor)
            probs = torch.softmax(output, dim=1)
            pred = output.argmax(dim=1).item()
            conf = probs[0][pred].item()

        return pred, conf

    def process(self, image_path: str) -> dict:
        """处理单张图片，返回所有 checkbox 的检测结果"""
        img = cv2.imdecode(np.fromfile(image_path, dtype=np.uint8), cv2.IMREAD_COLOR)
        if img is None:
            return {"error": f"Cannot read: {image_path}"}

        img = deskew(img)
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

        # 锚点定位
        anchor_y, scale = self._find_anchors(gray)
        if self.debug:
            print(f"  Anchor Y={anchor_y:.0f}, Scale={scale:.3f}")

        results = {}

        for field_name, field_def in CHECKBOX_MAP.items():
            positions = field_def["positions"]
            options = field_def["options"]
            field_type = field_def["type"]

            predictions = []

            for i, (cx, y_off) in enumerate(positions):
                cy = int(anchor_y + y_off * scale)
                crop = crop_checkbox(gray, cx, cy, size=32)

                if crop is None:
                    predictions.append((0, 0.0))
                    continue

                pred, conf = self.predict_crop(crop)
                predictions.append((pred, conf))

            # 决策逻辑
            checked_indices = self._decide(predictions, field_type)

            if self.debug:
                print(f"  {field_name}:")
                for i, ((pred, conf), (cx, y_off)) in enumerate(zip(predictions, positions)):
                    cy = int(anchor_y + y_off * scale)
                    marker = "*" if i in checked_indices else " "
                    opt_cn = field_def["options_cn"][i] if i < len(field_def["options_cn"]) else "?"
                    label_str = "checked" if pred == 1 else "unchecked"
                    print(f"    {marker} {opt_cn:12s} pred={label_str:9s} "
                          f"conf={conf:.3f} pos=({cx},{cy})")

            if field_type == "radio":
                results[field_name] = options[checked_indices[0]] if checked_indices else None
            else:
                results[field_name] = [options[i] for i in checked_indices]

        return results

    def _decide(self, predictions: list[tuple[int, float]], field_type: str) -> list[int]:
        """基于 CNN 预测结果做最终决策"""
        if field_type == "radio":
            # Radio: 选置信度最高的 checked 预测
            checked_candidates = [(i, conf) for i, (pred, conf) in enumerate(predictions)
                                  if pred == 1]

            if not checked_candidates:
                return []

            if len(checked_candidates) == 1:
                return [checked_candidates[0][0]]

            # 多个被预测为 checked：取置信度最高的
            checked_candidates.sort(key=lambda x: x[1], reverse=True)
            best_idx, best_conf = checked_candidates[0]
            second_conf = checked_candidates[1][1] if len(checked_candidates) > 1 else 0

            # 如果最高置信度不够突出，不做选择
            if best_conf < 0.6:
                return []
            if best_conf - second_conf < 0.1 and best_conf < 0.8:
                return []

            return [best_idx]

        else:
            # Multi-checkbox: 所有高置信度的 checked 预测
            checked = []
            for i, (pred, conf) in enumerate(predictions):
                if pred == 1 and conf >= 0.6:
                    checked.append(i)
            return checked

    def _find_anchors(self, gray: np.ndarray) -> tuple[float, float]:
        anchor1_match = _match_template_multi_scale(gray, self.tmpl_anchor1)
        anchor2_match = _match_template_multi_scale(gray, self.tmpl_anchor2)

        anchor1_y = float(ANCHOR_BASE_Y)
        scale = 1.0

        if anchor1_match is not None:
            anchor1_y = anchor1_match[1]
        if anchor2_match is not None:
            anchor2_y = anchor2_match[1]
            scale = (anchor2_y - anchor1_y) / BASE_SPAN

        return anchor1_y, scale


# ============================================================
# 输出格式化
# ============================================================

def to_cn_output(results: dict) -> dict:
    """转换为中文输出"""
    cn_results = {}
    for field_name, value in results.items():
        field_def = CHECKBOX_MAP.get(field_name, {})
        options = field_def.get("options", [])
        options_cn = field_def.get("options_cn", [])
        en_to_cn = dict(zip(options, options_cn))

        if value is None:
            cn_results[field_name] = None
        elif isinstance(value, list):
            cn_results[field_name] = [en_to_cn.get(v, v) for v in value]
        else:
            cn_results[field_name] = en_to_cn.get(value, value)

    return cn_results


# ============================================================
# 入口
# ============================================================

def main():
    base_dir = Path(r"d:\work\github\ocr")
    model_path = base_dir / "models" / "checkbox_cnn.pth"
    reference_path = base_dir / "微信图片_20260519194813_10_879.jpg"

    if not model_path.exists():
        print(f"ERROR: Model not found at {model_path}")
        print("Please run the following steps first:")
        print("  1. python checkbox_cnn_prepare.py")
        print("  2. python checkbox_cnn_train.py")
        return

    # 目标图片
    if len(sys.argv) > 1:
        target_image = sys.argv[1]
    else:
        target_image = str(base_dir / "new0520.jpg")

    print(f"Model: {model_path}")
    print(f"Reference: {reference_path}")
    print(f"Target: {target_image}")

    # 检测
    detector = CheckboxCNNDetector(
        model_path=str(model_path),
        reference_path=str(reference_path),
        debug=True,
    )

    print(f"\n{'='*60}")
    print(f"Processing: {Path(target_image).name}")
    print(f"{'='*60}")

    results = detector.process(target_image)

    if "error" in results:
        print(f"ERROR: {results['error']}")
        return

    # 输出结果
    print(f"\n{'='*60}")
    print("Results (EN):")
    for field, value in results.items():
        print(f"  {field:30s} = {value}")

    cn_results = to_cn_output(results)
    print(f"\nResults (CN):")
    for field, value in cn_results.items():
        print(f"  {field:30s} = {value}")

    # 保存 JSON
    output = {
        "form_name": "seed_cleaning_card",
        "file": Path(target_image).name,
        "method": "cnn_classifier",
        "checkboxes": results,
        "checkboxes_cn": cn_results,
    }

    output_path = base_dir / "output_cnn.json"
    output_path.write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"\nSaved to {output_path}")

    # 与 ground truth 对比（如果有）
    gt_path = base_dir / "new0520_view.json"
    if gt_path.exists() and "new0520" in target_image:
        _compare_with_ground_truth(results, gt_path)


def _compare_with_ground_truth(results: dict, gt_path: Path):
    """与标准答案对比"""
    gt_data = json.loads(gt_path.read_text(encoding="utf-8"))

    # 从 view.json 格式转换为内部格式
    gt_map = _parse_view_json(gt_data)

    print(f"\n{'='*60}")
    print("Comparison with ground truth:")
    print(f"{'='*60}")

    correct = 0
    total = 0
    for field_name in CHECKBOX_MAP:
        detected = results.get(field_name)
        expected = gt_map.get(field_name)
        match = detected == expected
        correct += int(match)
        total += 1
        marker = "OK" if match else "WRONG"
        print(f"  {marker:5s} {field_name:25s} detected={str(detected):30s} expected={str(expected)}")

    print(f"\n  Accuracy: {correct}/{total} ({correct/total*100:.0f}%)")


def _parse_view_json(data: dict) -> dict:
    """将 new0520_view.json 格式转换为检测结果格式"""
    sections = data.get("sections", {})

    cn_to_en = {}
    for field_name, field_def in CHECKBOX_MAP.items():
        for cn, en in zip(field_def.get("options_cn", []), field_def["options"]):
            cn_to_en[cn] = en

    result = {}

    # 包装袋类型
    bag = sections.get("清理登记", {}).get("包装袋类型", {})
    if isinstance(bag, dict):
        val = bag.get("value")
        result["bag_type"] = cn_to_en.get(val) if val else None
    else:
        result["bag_type"] = None

    # 纯净度
    purity = sections.get("清理登记", {}).get("纯净度(可多选)", {})
    if isinstance(purity, dict):
        vals = purity.get("value", [])
        result["purity"] = [cn_to_en.get(v, v) for v in (vals or [])]
    else:
        result["purity"] = []

    # 材料类型
    mat = sections.get("清理登记", {}).get("材料类型", {})
    if isinstance(mat, dict):
        val = mat.get("value")
        result["material_type"] = cn_to_en.get(val) if val else None
    else:
        result["material_type"] = None

    # 种子情况
    seed = sections.get("清理登记", {}).get("种子情况", {})
    if isinstance(seed, dict):
        vals = seed.get("value", [])
        result["seed_condition"] = [cn_to_en.get(v, v) for v in (vals or [])]
    else:
        result["seed_condition"] = []

    # 是否需要清理
    clean = sections.get("清理登记", {}).get("是否需要清理", {})
    if isinstance(clean, dict):
        val = clean.get("value")
        result["needs_cleaning"] = cn_to_en.get(val) if val else None
    else:
        result["needs_cleaning"] = None

    # 清理方法
    method = sections.get("清理登记", {}).get("清理方法", {})
    if isinstance(method, dict):
        val = method.get("value")
        result["cleaning_method"] = cn_to_en.get(val) if val else None
    else:
        result["cleaning_method"] = None

    # 清理结果
    res = sections.get("清理结果", {}).get("清理结果", {})
    if isinstance(res, dict):
        val = res.get("value")
        result["result_type"] = cn_to_en.get(val) if val else None
    else:
        result["result_type"] = None

    # 残渣量
    residue = sections.get("清理结果", {}).get("残渣量", {})
    if isinstance(residue, dict):
        val = residue.get("value")
        result["residue_amount"] = cn_to_en.get(val) if val else None
    else:
        result["residue_amount"] = None

    # X光取样数量
    xray = sections.get("清理结果", {}).get("X光取样数量", {})
    if isinstance(xray, dict):
        val = xray.get("value")
        result["xray_sample_quantity"] = val
    else:
        result["xray_sample_quantity"] = None

    # 转历史
    transfer = sections.get("清理结果", {}).get("转历史", {})
    if isinstance(transfer, dict):
        val = transfer.get("value")
        result["transfer_to_history"] = cn_to_en.get(val) if val else None
    else:
        result["transfer_to_history"] = None

    # 异常情况
    abnormal = sections.get("转历史原因", {}).get("异常情况", {})
    if isinstance(abnormal, dict):
        vals = abnormal.get("value", [])
        result["abnormal_conditions"] = [cn_to_en.get(v, v) for v in (vals or [])]
    else:
        result["abnormal_conditions"] = []

    return result


if __name__ == "__main__":
    main()
