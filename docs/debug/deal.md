DO $$
BEGIN
  IF to_regclass('public.dwd_forecast_wf_short_term_partition_2026_03') IS NULL THEN
    EXECUTE $q$
      CREATE TABLE public.dwd_forecast_wf_short_term_partition_2026_03
      PARTITION OF public.dwd_forecast_wf_short_term_partition
      FOR VALUES FROM ('2026-03-01 00:00:00') TO ('2026-04-01 00:00:00')
    $q$;
  END IF;

  IF to_regclass('public.dwd_forecast_wf_ultra_short_term_partition_2026_03') IS NULL THEN
    EXECUTE $q$
      CREATE TABLE public.dwd_forecast_wf_ultra_short_term_partition_2026_03
      PARTITION OF public.dwd_forecast_wf_ultra_short_term
      FOR VALUES FROM ('2026-03-01 00:00:00') TO ('2026-04-01 00:00:00')
    $q$;
  END IF;
END $$;
