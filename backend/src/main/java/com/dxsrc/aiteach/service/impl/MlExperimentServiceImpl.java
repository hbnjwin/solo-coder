package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Experiment;
import com.dxsrc.aiteach.mapper.ExperimentMapper;
import com.dxsrc.aiteach.service.MlExperimentService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class MlExperimentServiceImpl extends ServiceImpl<ExperimentMapper, Experiment> implements MlExperimentService {

    @Override
    public List<Experiment> list() {
        return super.list();
    }

    @Override
    public Experiment getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Experiment entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Experiment entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
