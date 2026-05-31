"""
Checkbox CNN 数据准备脚本

从已有图片中裁剪 checkbox 区域，根据 ground truth 标注为 checked/unchecked，
保存到 dataset/ 目录供训练使用。

用法:
    python checkbox_cnn_prepare.py

输出:
    dataset/checked/     - 被勾选的 checkbox 裁剪图
    dataset/unchecked/   - 未勾选的 checkbox 裁剪图
"""

import json
from pathlib import Path

import cv2
import numpy as np


# ============================================================
# 复用对齐逻辑（从 checkbox_omr_tmpl.py 提取核心部分）
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
        "positions": [(370, 2), (526, 2), (690, 4), (910, -15), (1044, 5)],
    },
    "purity": {
        "type": "checkbox_multi",
        "options": ["seeds", "branches_leaves", "insects", "soil_gravel", "other_seeds"],
        "positions": [(370, 84), (526, 86), (690, 67), (910, 67), (1044, 66)],
    },
    "material_type": {
        "type": "radio",
        "options": ["dry_fruit", "berry", "cleaned_seeds_or_fruit", "fleshy_fruit", "pure_seeds"],
        "positions": [(370, 168), (526, 168), (690, 148), (910, 148), (1044, 169)],
    },
    "seed_condition": {
        "type": "checkbox_multi",
        "options": ["fully_mature", "partially_mature", "immature",
                    "dispersed", "severely_insect_damaged", "severely_shriveled"],
        "positions": [(370, 230), (526, 231), (690, 251),
                      (370, 314), (690, 232), (1044, 232)],
    },
    "needs_cleaning": {
        "type": "radio",
        "options": ["yes", "no"],
        "positions": [(910, 331), (1044, 335)],
    },
    "cleaning_method": {
        "type": "radio",
        "options": ["berry_method", "dry_fruit_method", "fleshy_fruit_method", "manual_sorting"],
        "positions": [(370, 519), (526, 519), (690, 517), (910, 518)],
    },
    "result_type": {
        "type": "radio",
        "options": ["seeds", "fruit"],
        "positions": [(370, 593), (526, 593)],
    },
    "residue_amount": {
        "type": "radio",
        "options": ["none", "very_little", "small_amount"],
        "positions": [(370, 650), (526, 646), (690, 650)],
    },
    "xray_sample_quantity": {
        "type": "radio",
        "options": ["5", "10", "20", "30"],
        "positions": [(910, 793), (1044, 792), (910, 856), (1044, 856)],
    },
    "transfer_to_history": {
        "type": "radio",
        "options": ["no", "yes"],
        "positions": [(370, 848), (526, 848)],
    },
    "abnormal_conditions": {
        "type": "checkbox_multi",
        "options": ["numbering_disorder", "seeds_germinated", "no_seeds", "seeds_mixed",
                    "seeds_shriveled_or_immature", "seeds_severely_damaged",
                    "seeds_insect_or_mold_damaged", "other_reasons"],
        "positions": [(150, 902), (370, 902), (526, 902), (690, 902),
                      (150, 960), (526, 960), (690, 960), (910, 960)],
    },
}

# Ground truth 定义
GROUND_TRUTHS = {
    "微信图片_20260519194813_10_879.jpg": {
        "bag_type": "mesh_bag",
        "purity": ["seeds"],
        "material_type": "cleaned_seeds_or_fruit",
        "seed_condition": ["fully_mature"],
        "needs_cleaning": "yes",
        "cleaning_method": "dry_fruit_method",
        "result_type": "seeds",
        "residue_amount": "very_little",
        "xray_sample_quantity": "30",
        "transfer_to_history": None,
        "abnormal_conditions": [],
    },
    "微信图片_20260519194831_11_879.jpg": {
        "bag_type": "mesh_bag",
        "purity": ["seeds"],
        "material_type": "cleaned_seeds_or_fruit",
        "seed_condition": ["fully_mature"],
        "needs_cleaning": "yes",
        "cleaning_method": "dry_fruit_method",
        "result_type": "seeds",
        "residue_amount": "very_little",
        "xray_sample_quantity": "30",
        "transfer_to_history": None,
        "abnormal_conditions": [],
    },
    "new0520.jpg": {
        "bag_type": "cloth_bag",
        "purity": ["soil_gravel"],
        "material_type": "berry",
        "seed_condition": ["partially_mature"],
        "needs_cleaning": "yes",
        "cleaning_method": "fleshy_fruit_method",
        "result_type": "seeds",
        "residue_amount": "very_little",
        "xray_sample_quantity": "20",
        "transfer_to_history": None,
        "abnormal_conditions": [],
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


def find_anchors(gray, ref_gray):
    x1, x2, y1, y2 = ANCHOR1_REGION
    tmpl1 = ref_gray[y1:y2, x1:x2].copy()
    x1, x2, y1, y2 = ANCHOR2_REGION
    tmpl2 = ref_gray[y1:y2, x1:x2].copy()

    anchor1_match = _match_template_multi_scale(gray, tmpl1)
    anchor2_match = _match_template_multi_scale(gray, tmpl2)

    anchor1_y = float(ANCHOR_BASE_Y)
    scale = 1.0

    if anchor1_match is not None:
        anchor1_y = anchor1_match[1]
    if anchor2_match is not None:
        anchor2_y = anchor2_match[1]
        scale = (anchor2_y - anchor1_y) / BASE_SPAN

    return anchor1_y, scale


def crop_checkbox(gray: np.ndarray, cx: int, cy: int, size: int = 32) -> np.ndarray | None:
    """裁剪 checkbox 区域，输出固定 size x size 的灰度图"""
    half = size // 2
    h, w = gray.shape
    x1 = cx - half
    y1 = cy - half
    x2 = cx + half
    y2 = cy + half

    if x1 < 0 or y1 < 0 or x2 > w or y2 > h:
        # 边界处理：padding
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


def get_checked_indices(field_name: str, gt_value) -> set[int]:
    """根据 ground truth 值返回被勾选的 option 索引"""
    field_def = CHECKBOX_MAP[field_name]
    options = field_def["options"]

    if gt_value is None:
        return set()

    if isinstance(gt_value, list):
        indices = set()
        for v in gt_value:
            if v in options:
                indices.add(options.index(v))
        return indices
    else:
        if gt_value in options:
            return {options.index(gt_value)}
        return set()


def process_image(image_path: str, ref_gray: np.ndarray, gt: dict,
                  crop_size: int = 32) -> tuple[list, list]:
    """处理单张图片，返回 (crops, labels)"""
    img = cv2.imdecode(np.fromfile(image_path, dtype=np.uint8), cv2.IMREAD_COLOR)
    if img is None:
        print(f"  Cannot read: {image_path}")
        return [], []

    img = deskew(img)
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

    anchor_y, scale = find_anchors(gray, ref_gray)
    print(f"  Anchor Y={anchor_y:.0f}, Scale={scale:.3f}")

    crops = []
    labels = []

    for field_name, field_def in CHECKBOX_MAP.items():
        positions = field_def["positions"]
        checked_indices = get_checked_indices(field_name, gt.get(field_name))

        for i, (cx, y_off) in enumerate(positions):
            cy = int(anchor_y + y_off * scale)
            crop = crop_checkbox(gray, cx, cy, size=crop_size)

            if crop is not None:
                label = 1 if i in checked_indices else 0
                crops.append(crop)
                labels.append(label)

    return crops, labels


def main():
    base_dir = Path(r"d:\work\github\ocr")
    dataset_dir = base_dir / "dataset"
    checked_dir = dataset_dir / "checked"
    unchecked_dir = dataset_dir / "unchecked"

    checked_dir.mkdir(parents=True, exist_ok=True)
    unchecked_dir.mkdir(parents=True, exist_ok=True)

    # 参考图
    ref_path = base_dir / "微信图片_20260519194813_10_879.jpg"
    ref_img = cv2.imdecode(np.fromfile(str(ref_path), dtype=np.uint8), cv2.IMREAD_COLOR)
    ref_gray = cv2.cvtColor(ref_img, cv2.COLOR_BGR2GRAY)

    total_checked = 0
    total_unchecked = 0

    for filename, gt in GROUND_TRUTHS.items():
        image_path = str(base_dir / filename)
        print(f"\nProcessing: {filename}")

        crops, labels = process_image(image_path, ref_gray, gt)

        for idx, (crop, label) in enumerate(zip(crops, labels)):
            if label == 1:
                save_path = checked_dir / f"{Path(filename).stem}_{idx:03d}.png"
                total_checked += 1
            else:
                save_path = unchecked_dir / f"{Path(filename).stem}_{idx:03d}.png"
                total_unchecked += 1

            cv2.imencode('.png', crop)[1].tofile(str(save_path))

    print(f"\n{'='*50}")
    print(f"Dataset prepared:")
    print(f"  Checked:   {total_checked} samples")
    print(f"  Unchecked: {total_unchecked} samples")
    print(f"  Total:     {total_checked + total_unchecked} samples")
    print(f"  Saved to:  {dataset_dir}")
    print(f"\nNext step: python checkbox_cnn_train.py")


if __name__ == "__main__":
    main()
