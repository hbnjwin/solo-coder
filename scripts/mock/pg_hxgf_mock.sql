-- PostgreSQL mock data for:
--   public.hdrfcstdata_hxgf
--   public.hdrsfcstdata_hxgf
-- Date window aligned with config:
--   2026-03-01 00:00:00 ~ 2026-03-03 00:00:00

BEGIN;

CREATE TABLE IF NOT EXISTS public.hdrfcstdata_hxgf (
  id            integer PRIMARY KEY,
  record_id     integer,
  fcst_time     timestamp,
  fcst_nodetype smallint,
  limit_upper   double precision,
  limit_lower   double precision,
  p             double precision,
  daytype       smallint,
  is_modify     smallint,
  is_main       integer,
  powertype     integer
);

CREATE INDEX IF NOT EXISTS idx_hdrfcstdata_hxgf_fcst_time
  ON public.hdrfcstdata_hxgf (fcst_time);

CREATE TABLE IF NOT EXISTS public.hdrsfcstdata_hxgf (
  id            integer PRIMARY KEY,
  record_id     integer,
  fcst_time     timestamp,
  fcst_nodetype smallint,
  limit_upper   double precision,
  limit_lower   double precision,
  p             double precision,
  pointtype     smallint,
  is_modify     smallint,
  powertype     integer,
  UNIQUE (record_id, fcst_time)
);

CREATE INDEX IF NOT EXISTS idx_hdrsfcstdata_hxgf_fcst_time
  ON public.hdrsfcstdata_hxgf (fcst_time);

-- Idempotent cleanup in target range.
DELETE FROM public.hdrfcstdata_hxgf
WHERE fcst_time BETWEEN '2026-03-01 00:00:00' AND '2026-03-03 00:00:00';

DELETE FROM public.hdrsfcstdata_hxgf
WHERE fcst_time BETWEEN '2026-03-01 00:00:00' AND '2026-03-03 00:00:00';

INSERT INTO public.hdrfcstdata_hxgf (
  id, record_id, fcst_time, fcst_nodetype, limit_upper, limit_lower, p,
  daytype, is_modify, is_main, powertype
) VALUES
  (100001, 9001, '2026-03-01 00:00:00', 1, 100.0, 0.0, 18.6, 1, 0, 1, 1),
  (100002, 9002, '2026-03-01 06:00:00', 1, 100.0, 0.0, 22.3, 1, 0, 1, 1),
  (100003, 9003, '2026-03-01 12:00:00', 1, 100.0, 0.0, 35.7, 1, 0, 1, 1),
  (100004, 9004, '2026-03-01 18:00:00', 1, 100.0, 0.0, 28.1, 1, 0, 1, 1),
  (100005, 9005, '2026-03-02 00:00:00', 1, 100.0, 0.0, 19.8, 1, 0, 1, 1),
  (100006, 9006, '2026-03-02 06:00:00', 1, 100.0, 0.0, 24.2, 1, 0, 1, 1),
  (100007, 9007, '2026-03-02 12:00:00', 1, 100.0, 0.0, 33.4, 1, 0, 1, 1),
  (100008, 9008, '2026-03-02 18:00:00', 1, 100.0, 0.0, 26.9, 1, 0, 1, 1),
  (100009, 9009, '2026-03-03 00:00:00', 1, 100.0, 0.0, 20.5, 1, 0, 1, 1);

INSERT INTO public.hdrsfcstdata_hxgf (
  id, record_id, fcst_time, fcst_nodetype, limit_upper, limit_lower, p,
  pointtype, is_modify, powertype
) VALUES
  (200001, 9101, '2026-03-01 00:00:00', 1, 100.0, 0.0, 17.2, 1, 0, 1),
  (200002, 9102, '2026-03-01 04:00:00', 1, 100.0, 0.0, 21.8, 1, 0, 1),
  (200003, 9103, '2026-03-01 08:00:00', 1, 100.0, 0.0, 27.6, 1, 0, 1),
  (200004, 9104, '2026-03-01 12:00:00', 1, 100.0, 0.0, 31.3, 1, 0, 1),
  (200005, 9105, '2026-03-01 16:00:00', 1, 100.0, 0.0, 29.7, 1, 0, 1),
  (200006, 9106, '2026-03-01 20:00:00', 1, 100.0, 0.0, 24.9, 1, 0, 1),
  (200007, 9107, '2026-03-02 00:00:00', 1, 100.0, 0.0, 18.3, 1, 0, 1),
  (200008, 9108, '2026-03-02 12:00:00', 1, 100.0, 0.0, 34.1, 1, 0, 1),
  (200009, 9109, '2026-03-03 00:00:00', 1, 100.0, 0.0, 21.0, 1, 0, 1);

COMMIT;

-- Verify
SELECT COUNT(*) AS pg_short_term_rows
FROM public.hdrfcstdata_hxgf
WHERE fcst_time BETWEEN '2026-03-01 00:00:00' AND '2026-03-03 00:00:00';

SELECT COUNT(*) AS pg_super_short_term_rows
FROM public.hdrsfcstdata_hxgf
WHERE fcst_time BETWEEN '2026-03-01 00:00:00' AND '2026-03-03 00:00:00';
