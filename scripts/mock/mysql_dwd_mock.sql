-- MySQL mock data for:
--   dwd.dwd_forecast_wf_short_term_partition
--   dwd.dwd_forecast_wf_ultra_short_term
-- Date window aligned with config:
--   2026-03-01 00:00:00 ~ 2026-03-03 00:00:00

CREATE DATABASE IF NOT EXISTS dwd CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS dwd.dwd_forecast_wf_short_term_partition (
  id               BIGINT PRIMARY KEY,
  wf_id            BIGINT,
  tier             VARCHAR(32),
  data_type        VARCHAR(64),
  record_date      DATETIME,
  forecast_date    DATETIME,
  forecast_batch   VARCHAR(32),
  model_id         BIGINT,
  model_version    VARCHAR(64),
  model_source     VARCHAR(32),
  data_value       JSON,
  cumulative_value DECIMAL(20,4),
  is_backup        SMALLINT,
  create_time      DATETIME,
  update_time      DATETIME,
  source_batch     VARCHAR(32)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS dwd.dwd_forecast_wf_ultra_short_term (
  id              BIGINT PRIMARY KEY,
  wf_id           BIGINT,
  data_type       VARCHAR(64),
  `interval`      INT,
  record_date     DATETIME,
  forecast_date   DATETIME,
  forecast_batch  VARCHAR(32),
  forecast_length INT,
  model_id        BIGINT,
  model_version   VARCHAR(64),
  model_source    VARCHAR(32),
  data_value      JSON,
  is_backup       SMALLINT,
  create_time     DATETIME,
  update_time     DATETIME,
  source_batch    VARCHAR(32)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Idempotent cleanup in target range.
DELETE FROM dwd.dwd_forecast_wf_short_term_partition
WHERE forecast_date BETWEEN '2026-03-01 00:00:00' AND '2026-03-03 00:00:00';

DELETE FROM dwd.dwd_forecast_wf_ultra_short_term
WHERE forecast_date BETWEEN '2026-03-01 00:00:00' AND '2026-03-03 00:00:00';

INSERT INTO dwd.dwd_forecast_wf_short_term_partition (
  id, wf_id, tier, data_type, record_date, forecast_date, forecast_batch,
  model_id, model_version, model_source, data_value, cumulative_value,
  is_backup, create_time, update_time, source_batch
) VALUES
  (300001, 101, '0', 'power', '2026-03-01 00:00:00', '2026-03-01 00:00:00', '00',
   501, 'v1.0.0', 'forecast', JSON_ARRAY(18.6,19.1,20.3,22.0), 80.0000, 0, NOW(), NOW(), '00'),
  (300002, 101, '0', 'power', '2026-03-01 12:00:00', '2026-03-01 12:00:00', '12',
   501, 'v1.0.0', 'forecast', JSON_ARRAY(26.2,25.7,24.0,22.4), 98.3000, 0, NOW(), NOW(), '12'),
  (300003, 101, '0', 'power', '2026-03-02 00:00:00', '2026-03-02 00:00:00', '00',
   501, 'v1.0.1', 'forecast', JSON_ARRAY(21.5,22.9,24.4,26.1), 94.9000, 0, NOW(), NOW(), '00'),
  (300004, 101, '0', 'power', '2026-03-02 12:00:00', '2026-03-02 12:00:00', '12',
   501, 'v1.0.1', 'forecast', JSON_ARRAY(31.2,30.4,28.8,27.0), 117.4000, 0, NOW(), NOW(), '12'),
  (300005, 101, '0', 'power', '2026-03-03 00:00:00', '2026-03-03 00:00:00', '00',
   501, 'v1.0.2', 'forecast', JSON_ARRAY(20.8,21.3,22.7,24.1), 88.9000, 0, NOW(), NOW(), '00');

INSERT INTO dwd.dwd_forecast_wf_ultra_short_term (
  id, wf_id, data_type, `interval`, record_date, forecast_date, forecast_batch,
  forecast_length, model_id, model_version, model_source, data_value,
  is_backup, create_time, update_time, source_batch
) VALUES
  (400001, 101, 'power', 15, '2026-03-01 00:00:00', '2026-03-01 00:00:00', '00',
   4, 601, 'v2.0.0', 'forecast', JSON_ARRAY(18.0,18.5,19.3,20.2,21.1,22.0), 0, NOW(), NOW(), '00'),
  (400002, 101, 'power', 15, '2026-03-01 08:00:00', '2026-03-01 08:00:00', '08',
   4, 601, 'v2.0.0', 'forecast', JSON_ARRAY(23.4,24.1,24.9,25.8,26.2,26.8), 0, NOW(), NOW(), '08'),
  (400003, 101, 'power', 15, '2026-03-01 16:00:00', '2026-03-01 16:00:00', '16',
   4, 601, 'v2.0.1', 'forecast', JSON_ARRAY(28.1,27.6,27.0,26.5,25.9,25.2), 0, NOW(), NOW(), '16'),
  (400004, 101, 'power', 15, '2026-03-02 00:00:00', '2026-03-02 00:00:00', '00',
   4, 601, 'v2.0.1', 'forecast', JSON_ARRAY(19.2,19.7,20.4,21.3,22.1,22.8), 0, NOW(), NOW(), '00'),
  (400005, 101, 'power', 15, '2026-03-02 12:00:00', '2026-03-02 12:00:00', '12',
   4, 601, 'v2.0.2', 'forecast', JSON_ARRAY(30.1,30.5,29.8,28.9,28.1,27.4), 0, NOW(), NOW(), '12'),
  (400006, 101, 'power', 15, '2026-03-03 00:00:00', '2026-03-03 00:00:00', '00',
   4, 601, 'v2.0.2', 'forecast', JSON_ARRAY(21.0,21.4,22.0,22.7,23.5,24.0), 0, NOW(), NOW(), '00');

-- Verify
SELECT COUNT(*) AS mysql_short_term_rows
FROM dwd.dwd_forecast_wf_short_term_partition
WHERE forecast_date BETWEEN '2026-03-01 00:00:00' AND '2026-03-03 00:00:00';

SELECT COUNT(*) AS mysql_super_short_term_rows
FROM dwd.dwd_forecast_wf_ultra_short_term
WHERE forecast_date BETWEEN '2026-03-01 00:00:00' AND '2026-03-03 00:00:00';
