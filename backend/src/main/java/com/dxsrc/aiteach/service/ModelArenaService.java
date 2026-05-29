package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.ModelConfig;
import java.util.List;

public interface ModelArenaService {

    List<ModelConfig> list();

    ModelConfig getById(Long id);

    boolean save(ModelConfig entity);

    boolean update(ModelConfig entity);

    boolean removeById(Long id);
}
