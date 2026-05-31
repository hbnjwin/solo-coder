"""
YOLO Checkbox 评估脚本

在验证集上运行模型，输出 mAP、precision、recall 等指标。
"""

import argparse
from pathlib import Path

from ultralytics import YOLO


def evaluate(model_path: str, config: str = None, imgsz: int = 640):
    if config is None:
        config = str(Path(__file__).resolve().parent.parent / "config.yaml")

    model = YOLO(model_path)

    metrics = model.val(
        data=config,
        imgsz=imgsz,
        verbose=True,
        plots=True,
    )

    print("\n" + "=" * 50)
    print("Evaluation Results")
    print("=" * 50)
    print(f"  mAP50:      {metrics.box.map50:.4f}")
    print(f"  mAP50-95:   {metrics.box.map:.4f}")
    print(f"  Precision:  {metrics.box.mp:.4f}")
    print(f"  Recall:     {metrics.box.mr:.4f}")

    if hasattr(metrics.box, "ap_class_index"):
        class_names = {0: "unchecked", 1: "checked"}
        print("\nPer-class AP50:")
        for i, cls_idx in enumerate(metrics.box.ap_class_index):
            name = class_names.get(int(cls_idx), str(cls_idx))
            ap50 = metrics.box.ap50[i]
            print(f"  {name:12s}: {ap50:.4f}")

    return metrics


def main():
    parser = argparse.ArgumentParser(description="Evaluate YOLO checkbox detector")
    parser.add_argument("--model", type=str, default=None, help="Path to best.pt")
    parser.add_argument("--config", type=str, default=None, help="Path to config.yaml")
    parser.add_argument("--imgsz", type=int, default=640)
    args = parser.parse_args()

    if args.model is None:
        default_weights = Path(__file__).resolve().parent.parent / "runs" / "checkbox_det" / "weights" / "best.pt"
        if default_weights.exists():
            args.model = str(default_weights)
        else:
            print(f"No model found at: {default_weights}")
            print("Train first with: python scripts/train.py")
            return

    evaluate(args.model, args.config, args.imgsz)


if __name__ == "__main__":
    main()
