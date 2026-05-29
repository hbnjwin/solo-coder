package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Workflow;
import com.dxsrc.aiteach.mapper.WorkflowMapper;
import com.dxsrc.aiteach.service.WorkflowDebugService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class WorkflowDebugServiceImpl extends ServiceImpl<WorkflowMapper, Workflow> implements WorkflowDebugService {

    @Override
    public List<Workflow> list() {
        return super.list();
    }

    @Override
    public Workflow getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Workflow entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Workflow entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
