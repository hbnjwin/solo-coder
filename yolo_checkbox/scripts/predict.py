"""
YOLO Checkbox 推理脚本

加载训练好的模型，对图片做 checkbox 检测，
输出与 checkbox_omr.py 相同格式的 JSON 结果。
"""

import json
import argparse
from pathlib import Path

import cv2
import numpy as np
from ultralytics import YOLO


CLASS_NAMES = {0: "unchecked", 1: "checked"}


def predict(image_path: str, model_path: str, conf: float = 0.25,
            imgsz: int = 640, save_viz: bool = False) -> list[dict]:
    model = YOLO(model_path)

    results = model.predict(
        source=image_path,
        conf=conf,
        imgsz=imgsz,
        save=save_viz,
        verbose=False,
    )

    detections = []
    for r in results:
        boxes = r.boxes
        for i in range(len(boxes)):
            cls_id = int(boxes.cls[i].item())
            confidence = float(boxes.conf[i].item())
            x1, y1, x2, y2 = boxes.xyxy[i].tolist()
            cx = (x1 + x2) / 2
            cy = (y1 + y2) / 2

            detections.append({
                "class": CLASS_NAMES.get(cls_id, str(cls_id)),
                "class_id": cls_id,
                "confidence": round(confidence, 4),
                "bbox": [round(x1, 1), round(y1, 1), round(x2, 1), round(y2, 1)],
                "center": [round(cx, 1), round(cy, 1)],
            })

    detections.sort(key=lambda d: (d["center"][1], d["center"][0]))
    return detections


def main():
    parser = argparse.ArgumentParser(description="YOLO checkbox prediction")
    parser.add_argument("image", type=str, help="Image path or directory")
    parser.add_argument("--model", type=str, default=None,
                        help="Path to trained weights (best.pt)")
    parser.add_argument("--conf", type=float, default=0.25, help="Confidence threshold")
    parser.add_argument("--imgsz", type=int, default=640, help="Inference image size")
    parser.add_argument("--output", type=str, default="output_yolo.json", help="Output JSON path")
    parser.add_argument("--save-viz", action="store_true", help="Save visualization images")
    args = parser.parse_args()

    if args.model is None:
        default_weights = Path(__file__).resolve().parent.parent / "runs" / "checkbox_det" / "weights" / "best.pt"
        if default_weights.exists():
            args.model = str(default_weights)
        else:
            print(f"No model found. Train first or specify --model path.")
            print(f"Expected: {default_weights}")
            return

    image_path = Path(args.image)
    if image_path.is_dir():
        images = list(image_path.glob("*.jpg")) + list(image_path.glob("*.png"))
    else:
        images = [image_path]

    all_results = []
    for img_path in images:
        print(f"Processing: {img_path.name}")
        detections = predict(str(img_path), args.model, args.conf, args.imgsz, args.save_viz)

        checked_count = sum(1 for d in detections if d["class"] == "checked")
        unchecked_count = sum(1 for d in detections if d["class"] == "unchecked")
        print(f"  Found {len(detections)} checkboxes: {checked_count} checked, {unchecked_count} unchecked")

        all_results.append({
            "file": img_path.name,
            "detections": detections,
            "summary": {
                "total": len(detections),
                "checked": checked_count,
                "unchecked": unchecked_count,
            }
        })

    output = {"model": args.model, "results": all_results}
    Path(args.output).write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"\nSaved to {args.output}")


if __name__ == "__main__":
    main()
