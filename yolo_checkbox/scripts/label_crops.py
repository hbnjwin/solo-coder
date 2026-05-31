"""
自动标注工具：利用 checkbox_omr.py 中的固定坐标 + ground truth，
为现有图片生成 YOLO 格式标注文件。

YOLO 标注格式：每行 class_id cx cy w h（归一化到 0~1）
- class 0: unchecked
- class 1: checked
"""

import sys
from pathlib import Path

import cv2
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from checkbox_omr import (
    CHECKBOX_MAP, ANCHOR_BASE_Y, ANCHOR2_LABEL_Y, BASE_SPAN,
    deskew, compute_homography, SeedCardOMR,
)


GROUND_TRUTH = {
    "bag_type": {"mesh_bag"},
    "purity": {"seeds"},
    "material_type": set(),
    "seed_condition": {"fully_mature"},
    "needs_cleaning": {"yes"},
    "cleaning_method": {"dry_fruit_method"},
    "result_type": {"seeds"},
    "residue_amount": {"very_little"},
    "xray_sample_quantity": {"30"},
    "transfer_to_history": set(),
    "abnormal_conditions": set(),
}

BOX_SIZE = 25


def generate_labels(image_path: str, output_label_path: str,
                    anchor_y: float = None, scale: float = 1.0):
    img = cv2.imdecode(np.fromfile(image_path, dtype=np.uint8), cv2.IMREAD_COLOR)
    if img is None:
        print(f"Cannot read: {image_path}")
        return

    img = deskew(img)
    img_h, img_w = img.shape[:2]

    if anchor_y is None:
        import easyocr
        reader = easyocr.Reader(["ch_sim", "en"], gpu=False)
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
        img_bgr = cv2.cvtColor(gray, cv2.COLOR_GRAY2BGR)
        ocr_results = reader.readtext(img_bgr)
        for bbox, text, conf in ocr_results:
            if "包装袋" in text or "袋类型" in text:
                ys = [p[1] for p in bbox]
                anchor_y = (min(ys) + max(ys)) / 2
                break
        if anchor_y is None:
            anchor_y = ANCHOR_BASE_Y

    lines = []

    for field_name, field_def in CHECKBOX_MAP.items():
        options = field_def["options"]
        positions = field_def["positions"]
        checked_set = GROUND_TRUTH.get(field_name, set())

        for i, (cx, y_off) in enumerate(positions):
            cy = int(anchor_y + y_off * scale)

            if cx < 0 or cx >= img_w or cy < 0 or cy >= img_h:
                continue

            opt = options[i]
            class_id = 1 if opt in checked_set else 0

            norm_cx = cx / img_w
            norm_cy = cy / img_h
            norm_w = BOX_SIZE / img_w
            norm_h = BOX_SIZE / img_h

            lines.append(f"{class_id} {norm_cx:.6f} {norm_cy:.6f} {norm_w:.6f} {norm_h:.6f}")

    Path(output_label_path).parent.mkdir(parents=True, exist_ok=True)
    Path(output_label_path).write_text("\n".join(lines), encoding="utf-8")
    print(f"Generated {len(lines)} labels -> {output_label_path}")


def main():
    project_root = Path(__file__).resolve().parent.parent.parent
    data_dir = Path(__file__).resolve().parent.parent / "data"

    images = [
        project_root / "微信图片_20260519194813_10_879.jpg",
        project_root / "微信图片_20260519194831_11_879.jpg",
    ]

    for img_path in images:
        if not img_path.exists():
            print(f"Image not found: {img_path}")
            continue

        stem = img_path.stem
        img_dest = data_dir / "images" / "train" / f"{stem}.jpg"
        label_dest = data_dir / "labels" / "train" / f"{stem}.txt"

        img_dest.parent.mkdir(parents=True, exist_ok=True)
        label_dest.parent.mkdir(parents=True, exist_ok=True)

        import shutil
        shutil.copy2(str(img_path), str(img_dest))

        generate_labels(str(img_path), str(label_dest))

    print("\nDone. Copy some images to data/images/val/ and labels to data/labels/val/ for validation.")


if __name__ == "__main__":
    main()
