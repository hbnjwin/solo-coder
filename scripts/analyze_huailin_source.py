#!/usr/bin/env python3
import argparse
import csv
import json
import math
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Dict, List, Optional, Tuple


TZ_CST = timezone(timedelta(hours=8))
SHORT_TYPES = {
    "fore_wf_short_power",
    "fore_wf_short_power_original",
    "fore_wf_short_power_lower",
    "fore_wf_short_power_upper",
    "fore_wf_short_theory",
}


def load_report_rows(path: Path) -> List[Dict[str, str]]:
    with path.open("r", encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f))


def load_vendor_rows(path: Path) -> List[Dict[str, str]]:
    with path.open("r", encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f))


def parse_data_value(raw: str) -> Optional[List[float]]:
    try:
        return [float(x) for x in json.loads(raw)]
    except Exception:
        return None


def epoch_to_local_str(ts: int) -> str:
    return datetime.fromtimestamp(ts, tz=TZ_CST).strftime("%Y-%m-%d %H:%M:%S")


def build_report_series(rows: List[Dict[str, str]], start_batch: str) -> List[Tuple[int, float]]:
    out = []
    for row in rows:
        if row.get("start_batch") != start_batch:
            continue
        out.append((int(row["forecast_time"]), float(row["power"])))
    out.sort(key=lambda x: x[0])
    return out


def fit_metrics(x: List[float], y: List[float]) -> Tuple[float, float, float, float, float, float]:
    n = len(x)
    rmse_direct = math.sqrt(sum((yy - xx) ** 2 for xx, yy in zip(x, y)) / n)

    xx = sum(xx * xx for xx in x)
    if xx == 0:
        k = 0.0
        rmse_scale = float("inf")
    else:
        k = sum(xx * yy for xx, yy in zip(x, y)) / xx
        rmse_scale = math.sqrt(sum((yy - k * xx) ** 2 for xx, yy in zip(x, y)) / n)

    mx = sum(x) / n
    my = sum(y) / n
    sxx = sum((xx - mx) ** 2 for xx in x)
    if sxx == 0:
        a = 0.0
        b = my
    else:
        a = sum((xx - mx) * (yy - my) for xx, yy in zip(x, y)) / sxx
        b = my - a * mx
    pred = [a * xx + b for xx in x]
    rmse_affine = math.sqrt(sum((yy - pp) ** 2 for yy, pp in zip(y, pred)) / n)

    syy = sum((yy - my) ** 2 for yy in y)
    sxy = sum((xx - mx) * (yy - my) for xx, yy in zip(x, y))
    corr = sxy / math.sqrt(sxx * syy) if sxx > 0 and syy > 0 else 0.0
    return rmse_direct, rmse_scale, rmse_affine, k, a, b, corr


def main() -> None:
    parser = argparse.ArgumentParser(description="Trace report power vs vendor short-term CSV.")
    parser.add_argument("--report-csv", required=True, type=Path)
    parser.add_argument("--vendor-csv", required=True, type=Path)
    parser.add_argument("--target-ts", required=True, type=int)
    parser.add_argument("--start-batch", default="1")
    parser.add_argument("--forecast-date", default="")
    parser.add_argument("--out-dir", default="output/analysis", type=Path)
    parser.add_argument("--top-n", default=50, type=int)
    args = parser.parse_args()

    report_rows = load_report_rows(args.report_csv)
    vendor_rows = load_vendor_rows(args.vendor_csv)
    series = build_report_series(report_rows, args.start_batch)
    if not series:
        raise SystemExit(f"no report rows for start_batch={args.start_batch}")

    y = [v for _, v in series]
    ts_index = {ts: idx for idx, (ts, _) in enumerate(series)}
    if args.target_ts not in ts_index:
        raise SystemExit(f"target timestamp {args.target_ts} not found in report series")
    target_idx = ts_index[args.target_ts]

    local_day = datetime.fromtimestamp(args.target_ts, tz=TZ_CST).strftime("%Y-%m-%d")
    target_forecast_date = args.forecast_date or f"{local_day} 00:00:00"

    report_target_rows = [r for r in report_rows if int(r["forecast_time"]) == args.target_ts]

    fit_rows = []
    point_rows = []
    exact_hits = 0
    nearest = (float("inf"), None)

    for row in vendor_rows:
        if row.get("data_type") not in SHORT_TYPES:
            continue
        arr = parse_data_value(row.get("data_value", ""))
        if not arr:
            continue

        for i, v in enumerate(arr):
            d = abs(v - float(report_target_rows[0]["power"]))
            if d < nearest[0]:
                nearest = (d, (row, i, v))
            if d == 0.0:
                exact_hits += 1

        if row.get("forecast_date") != target_forecast_date:
            continue

        for start in (0, 1, 2):
            x = arr[start : start + len(y)]
            if len(x) != len(y):
                continue
            rmse_direct, rmse_scale, rmse_affine, k, a, b, corr = fit_metrics(x, y)
            fit_rows.append(
                {
                    "rmse_direct": rmse_direct,
                    "rmse_scale": rmse_scale,
                    "rmse_affine": rmse_affine,
                    "corr": corr,
                    "k": k,
                    "a": a,
                    "b": b,
                    "target_vendor": x[target_idx],
                    "target_affine": a * x[target_idx] + b,
                    "target_report": y[target_idx],
                    "start": start,
                    "data_type": row["data_type"],
                    "forecast_date": row["forecast_date"],
                    "record_date": row["record_date"],
                    "forecast_batch": row["forecast_batch"],
                    "source_batch": row.get("source_batch", ""),
                    "id": row["id"],
                }
            )

        if row.get("data_type") == "fore_wf_short_power":
            idx = int((args.target_ts - int(datetime.strptime(row["forecast_date"], "%Y-%m-%d %H:%M:%S").replace(tzinfo=TZ_CST).timestamp())) / 900)
            val = arr[idx] if 0 <= idx < len(arr) else None
            point_rows.append(
                {
                    "forecast_batch": row["forecast_batch"],
                    "record_date": row["record_date"],
                    "forecast_date": row["forecast_date"],
                    "source_batch": row.get("source_batch", ""),
                    "idx": idx,
                    "vendor_power": val,
                    "id": row["id"],
                }
            )

    fit_rows.sort(key=lambda r: r["rmse_affine"])
    point_rows.sort(key=lambda r: (r["record_date"], r["forecast_batch"]), reverse=True)

    args.out_dir.mkdir(parents=True, exist_ok=True)
    fit_path = args.out_dir / "huailin_best_fits.csv"
    point_path = args.out_dir / "huailin_point_candidates.csv"
    summary_path = args.out_dir / "huailin_summary.txt"

    with fit_path.open("w", encoding="utf-8", newline="") as f:
        w = csv.DictWriter(
            f,
            fieldnames=[
                "rmse_direct",
                "rmse_scale",
                "rmse_affine",
                "corr",
                "k",
                "a",
                "b",
                "target_vendor",
                "target_affine",
                "target_report",
                "start",
                "data_type",
                "forecast_date",
                "record_date",
                "forecast_batch",
                "source_batch",
                "id",
            ],
        )
        w.writeheader()
        for row in fit_rows[: args.top_n]:
            w.writerow(row)

    with point_path.open("w", encoding="utf-8", newline="") as f:
        w = csv.DictWriter(
            f,
            fieldnames=[
                "forecast_batch",
                "record_date",
                "forecast_date",
                "source_batch",
                "idx",
                "vendor_power",
                "id",
            ],
        )
        w.writeheader()
        for row in point_rows:
            w.writerow(row)

    with summary_path.open("w", encoding="utf-8") as f:
        f.write(f"target_ts={args.target_ts} ({epoch_to_local_str(args.target_ts)} CST)\n")
        f.write(f"target_forecast_date={target_forecast_date}\n")
        f.write(f"report_rows_at_target={len(report_target_rows)}\n")
        for r in report_target_rows:
            f.write(
                "report_row: "
                + ", ".join(
                    [
                        f"power={r['power']}",
                        f"start_time={r['start_time']}",
                        f"start_batch={r['start_batch']}",
                        f"pre_qrts={r['pre_qrts']}",
                        f"start_date={r['start_date']}",
                        f"create_time={r['create_time']}",
                    ]
                )
                + "\n"
            )
        f.write(f"fit_candidates={len(fit_rows)}\n")
        if fit_rows:
            top = fit_rows[0]
            f.write(
                "best_fit: "
                + ", ".join(
                    [
                        f"rmse_affine={top['rmse_affine']:.6f}",
                        f"corr={top['corr']:.6f}",
                        f"a={top['a']:.6f}",
                        f"b={top['b']:.6f}",
                        f"data_type={top['data_type']}",
                        f"forecast_batch={top['forecast_batch']}",
                        f"record_date={top['record_date']}",
                        f"source_batch={top['source_batch']}",
                    ]
                )
                + "\n"
            )
        f.write(f"exact_value_hits_in_vendor_arrays={exact_hits}\n")
        if nearest[1] is not None:
            row, idx, val = nearest[1]
            f.write(
                "nearest_value: "
                + ", ".join(
                    [
                        f"delta={nearest[0]:.6f}",
                        f"value={val}",
                        f"idx={idx}",
                        f"data_type={row['data_type']}",
                        f"forecast_date={row['forecast_date']}",
                        f"record_date={row['record_date']}",
                        f"forecast_batch={row['forecast_batch']}",
                        f"source_batch={row.get('source_batch', '')}",
                        f"id={row['id']}",
                    ]
                )
                + "\n"
            )

    print(f"summary: {summary_path}")
    print(f"best fits: {fit_path}")
    print(f"point candidates: {point_path}")


if __name__ == "__main__":
    main()
