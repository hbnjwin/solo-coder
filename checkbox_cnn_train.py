"""
Checkbox CNN 训练脚本

使用 PyTorch 训练一个轻量 CNN 分类器，判断 checkbox 是否被勾选。
通过重度数据增强弥补样本不足（3张图 ≈ 90 个样本 → 增强后数千个）。

用法:
    python checkbox_cnn_train.py

输入:
    dataset/checked/     - 被勾选的 checkbox 裁剪图
    dataset/unchecked/   - 未勾选的 checkbox 裁剪图

输出:
    models/checkbox_cnn.pth  - 训练好的模型权重
"""

from pathlib import Path

import cv2
import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import Dataset, DataLoader, WeightedRandomSampler


# ============================================================
# 数据集
# ============================================================

class CheckboxDataset(Dataset):
    """Checkbox 数据集，支持在线数据增强"""

    def __init__(self, data_dir: str, augment: bool = False, target_size: int = 32):
        self.samples = []
        self.labels = []
        self.augment = augment
        self.target_size = target_size

        checked_dir = Path(data_dir) / "checked"
        unchecked_dir = Path(data_dir) / "unchecked"

        for img_path in checked_dir.glob("*.png"):
            img = cv2.imdecode(np.fromfile(str(img_path), dtype=np.uint8), cv2.IMREAD_GRAYSCALE)
            if img is not None:
                self.samples.append(img)
                self.labels.append(1)

        for img_path in unchecked_dir.glob("*.png"):
            img = cv2.imdecode(np.fromfile(str(img_path), dtype=np.uint8), cv2.IMREAD_GRAYSCALE)
            if img is not None:
                self.samples.append(img)
                self.labels.append(0)

        print(f"  Loaded {len(self.samples)} samples "
              f"(checked={sum(self.labels)}, unchecked={len(self.labels)-sum(self.labels)})")

    def __len__(self):
        if self.augment:
            return len(self.samples) * 20
        return len(self.samples)

    def __getitem__(self, idx):
        real_idx = idx % len(self.samples)
        img = self.samples[real_idx].copy()
        label = self.labels[real_idx]

        if self.augment:
            img = self._augment(img)

        img = cv2.resize(img, (self.target_size, self.target_size), interpolation=cv2.INTER_AREA)
        tensor = torch.from_numpy(img).float().unsqueeze(0) / 255.0
        return tensor, label

    def _augment(self, img: np.ndarray) -> np.ndarray:
        h, w = img.shape[:2]

        # 随机旋转 ±5°
        if np.random.random() < 0.5:
            angle = np.random.uniform(-5, 5)
            M = cv2.getRotationMatrix2D((w / 2, h / 2), angle, 1.0)
            img = cv2.warpAffine(img, M, (w, h), borderMode=cv2.BORDER_REPLICATE)

        # 随机平移 ±3px
        if np.random.random() < 0.5:
            dx = np.random.randint(-3, 4)
            dy = np.random.randint(-3, 4)
            M = np.float32([[1, 0, dx], [0, 1, dy]])
            img = cv2.warpAffine(img, M, (w, h), borderMode=cv2.BORDER_REPLICATE)

        # 随机亮度调整
        if np.random.random() < 0.5:
            delta = np.random.randint(-30, 31)
            img = np.clip(img.astype(np.int16) + delta, 0, 255).astype(np.uint8)

        # 随机对比度
        if np.random.random() < 0.3:
            alpha = np.random.uniform(0.8, 1.2)
            img = np.clip(img.astype(np.float32) * alpha, 0, 255).astype(np.uint8)

        # 随机高斯噪声
        if np.random.random() < 0.3:
            noise = np.random.normal(0, 5, img.shape).astype(np.int16)
            img = np.clip(img.astype(np.int16) + noise, 0, 255).astype(np.uint8)

        # 随机高斯模糊
        if np.random.random() < 0.2:
            ksize = np.random.choice([3, 5])
            img = cv2.GaussianBlur(img, (ksize, ksize), 0)

        # 随机缩放 ±10%
        if np.random.random() < 0.3:
            scale = np.random.uniform(0.9, 1.1)
            new_size = int(w * scale)
            img = cv2.resize(img, (new_size, new_size), interpolation=cv2.INTER_LINEAR)
            if new_size > w:
                start = (new_size - w) // 2
                img = img[start:start + h, start:start + w]
            elif new_size < w:
                pad = (w - new_size) // 2
                img = cv2.copyMakeBorder(img, pad, w - new_size - pad,
                                         pad, w - new_size - pad,
                                         cv2.BORDER_REPLICATE)

        return img

    def get_class_weights(self) -> torch.Tensor:
        """计算类别权重用于处理不平衡"""
        n_checked = sum(self.labels)
        n_unchecked = len(self.labels) - n_checked
        if n_checked == 0 or n_unchecked == 0:
            return torch.ones(2)
        w_checked = len(self.labels) / (2 * n_checked)
        w_unchecked = len(self.labels) / (2 * n_unchecked)
        return torch.tensor([w_unchecked, w_checked])


# ============================================================
# CNN 模型
# ============================================================

class CheckboxCNN(nn.Module):
    """
    轻量 CNN：3 层卷积 + 2 层全连接
    输入: 1x32x32 灰度图
    输出: 2 类 (unchecked, checked)
    参数量: ~15K，CPU 推理 < 1ms
    """

    def __init__(self):
        super().__init__()
        self.features = nn.Sequential(
            nn.Conv2d(1, 16, 3, padding=1),
            nn.BatchNorm2d(16),
            nn.ReLU(inplace=True),
            nn.MaxPool2d(2),  # 16x16

            nn.Conv2d(16, 32, 3, padding=1),
            nn.BatchNorm2d(32),
            nn.ReLU(inplace=True),
            nn.MaxPool2d(2),  # 8x8

            nn.Conv2d(32, 64, 3, padding=1),
            nn.BatchNorm2d(64),
            nn.ReLU(inplace=True),
            nn.AdaptiveAvgPool2d(4),  # 4x4
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
# 训练
# ============================================================

def train(data_dir: str = "dataset", epochs: int = 80, lr: float = 0.001,
          batch_size: int = 32, model_dir: str = "models"):
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    print(f"Device: {device}")

    # 数据集
    print("\nLoading training data (with augmentation)...")
    train_dataset = CheckboxDataset(data_dir, augment=True)

    if len(train_dataset.samples) == 0:
        print("ERROR: No training data found. Run checkbox_cnn_prepare.py first.")
        return

    # 处理类别不平衡：使用 WeightedRandomSampler
    labels = train_dataset.labels
    class_counts = [labels.count(0), labels.count(1)]
    sample_weights = [1.0 / class_counts[l] for l in labels]
    # 扩展到增强后的长度
    sample_weights_full = sample_weights * 20
    sampler = WeightedRandomSampler(sample_weights_full, len(train_dataset), replacement=True)

    train_loader = DataLoader(train_dataset, batch_size=batch_size, sampler=sampler,
                              num_workers=0, pin_memory=True)

    # 验证集（不增强）
    print("Loading validation data (no augmentation)...")
    val_dataset = CheckboxDataset(data_dir, augment=False)
    val_loader = DataLoader(val_dataset, batch_size=batch_size, shuffle=False, num_workers=0)

    # 模型
    model = CheckboxCNN().to(device)
    param_count = sum(p.numel() for p in model.parameters())
    print(f"\nModel parameters: {param_count:,}")

    # 损失函数（加权）
    class_weights = train_dataset.get_class_weights().to(device)
    criterion = nn.CrossEntropyLoss(weight=class_weights)

    optimizer = optim.Adam(model.parameters(), lr=lr, weight_decay=1e-4)
    scheduler = optim.lr_scheduler.CosineAnnealingLR(optimizer, T_max=epochs)

    # 训练循环
    best_acc = 0.0
    model_path = Path(model_dir)
    model_path.mkdir(parents=True, exist_ok=True)

    print(f"\nTraining for {epochs} epochs...")
    print(f"{'Epoch':>5} {'Loss':>8} {'Train Acc':>10} {'Val Acc':>8} {'LR':>10}")
    print("-" * 50)

    for epoch in range(epochs):
        model.train()
        running_loss = 0.0
        correct = 0
        total = 0

        for inputs, targets in train_loader:
            inputs = inputs.to(device)
            targets = torch.tensor(targets).long().to(device)

            optimizer.zero_grad()
            outputs = model(inputs)
            loss = criterion(outputs, targets)
            loss.backward()
            optimizer.step()

            running_loss += loss.item()
            _, predicted = outputs.max(1)
            total += targets.size(0)
            correct += predicted.eq(targets).sum().item()

        scheduler.step()
        train_acc = correct / total

        # 验证
        model.eval()
        val_correct = 0
        val_total = 0
        with torch.no_grad():
            for inputs, targets in val_loader:
                inputs = inputs.to(device)
                targets = torch.tensor(targets).long().to(device)
                outputs = model(inputs)
                _, predicted = outputs.max(1)
                val_total += targets.size(0)
                val_correct += predicted.eq(targets).sum().item()

        val_acc = val_correct / val_total if val_total > 0 else 0

        if (epoch + 1) % 10 == 0 or epoch == 0:
            lr_now = scheduler.get_last_lr()[0]
            print(f"{epoch+1:>5} {running_loss/len(train_loader):>8.4f} "
                  f"{train_acc:>10.1%} {val_acc:>8.1%} {lr_now:>10.6f}")

        if val_acc >= best_acc:
            best_acc = val_acc
            torch.save({
                'model_state_dict': model.state_dict(),
                'accuracy': best_acc,
                'epoch': epoch,
                'class_names': ['unchecked', 'checked'],
            }, model_path / "checkbox_cnn.pth")

    print(f"\nTraining complete. Best validation accuracy: {best_acc:.1%}")
    print(f"Model saved to: {model_path / 'checkbox_cnn.pth'}")

    # 详细验证结果
    print(f"\nDetailed validation results:")
    model.eval()
    tp = fp = tn = fn = 0
    with torch.no_grad():
        for inputs, targets in val_loader:
            inputs = inputs.to(device)
            targets_t = torch.tensor(targets).long().to(device)
            outputs = model(inputs)
            _, predicted = outputs.max(1)
            for pred, gt in zip(predicted.cpu().numpy(), targets):
                if gt == 1 and pred == 1:
                    tp += 1
                elif gt == 0 and pred == 1:
                    fp += 1
                elif gt == 0 and pred == 0:
                    tn += 1
                else:
                    fn += 1

    precision = tp / (tp + fp) if (tp + fp) > 0 else 0
    recall = tp / (tp + fn) if (tp + fn) > 0 else 0
    f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0

    print(f"  TP={tp} FP={fp} TN={tn} FN={fn}")
    print(f"  Precision: {precision:.3f}")
    print(f"  Recall:    {recall:.3f}")
    print(f"  F1:        {f1:.3f}")


if __name__ == "__main__":
    train(
        data_dir=r"d:\work\github\ocr\dataset",
        model_dir=r"d:\work\github\ocr\models",
        epochs=80,
        batch_size=32,
        lr=0.001,
    )
