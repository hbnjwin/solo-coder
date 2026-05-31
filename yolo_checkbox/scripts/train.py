"""
YOLO Checkbox 训练脚本

使用 ultralytics YOLOv8n 训练 checkbox 检测模型。
类别：unchecked (0), checked (1)
"""

from pathlib import Path
from ultralytics import YOLO


def train(
    config: str = None,
    model: str = "yolov8n.pt",
    epochs: int = 100,
    imgsz: int = 640,
    batch: int = 16,
    device: str = "",
    project: str = None,
    name: str = "checkbox_det",
):
    if config is None:
        config = str(Path(__file__).resolve().parent.parent / "config.yaml")
    if project is None:
        project = str(Path(__file__).resolve().parent.parent / "runs")

    model = YOLO(model)

    model.train(
        data=config,
        epochs=epochs,
        imgsz=imgsz,
        batch=batch,
        device=device if device else None,
        project=project,
        name=name,
        exist_ok=True,
        patience=20,
        save=True,
        plots=True,
    )

    print(f"\nTraining complete. Weights saved to: {project}/{name}/weights/")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Train YOLO checkbox detector")
    parser.add_argument("--config", type=str, default=None, help="Path to config.yaml")
    parser.add_argument("--model", type=str, default="yolov8n.pt", help="Pretrained model")
    parser.add_argument("--epochs", type=int, default=100)
    parser.add_argument("--imgsz", type=int, default=640)
    parser.add_argument("--batch", type=int, default=16)
    parser.add_argument("--device", type=str, default="", help="cuda device or cpu")
    args = parser.parse_args()

    train(
        config=args.config,
        model=args.model,
        epochs=args.epochs,
        imgsz=args.imgsz,
        batch=args.batch,
        device=args.device,
    )
