package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Workflow;
import java.util.List;

public interface WorkflowDebugService {

    List<Workflow> list();

    Workflow getById(Long id);

    boolean save(Workflow entity);

    boolean update(Workflow entity);

    boolean removeById(Long id);
}
