package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Experiment;
import java.util.List;

public interface MlExperimentService {

    List<Experiment> list();

    Experiment getById(Long id);

    boolean save(Experiment entity);

    boolean update(Experiment entity);

    boolean removeById(Long id);
}
