package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.ModelConfig;
import com.dxsrc.aiteach.mapper.ModelConfigMapper;
import com.dxsrc.aiteach.service.ModelArenaService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class ModelArenaServiceImpl extends ServiceImpl<ModelConfigMapper, ModelConfig> implements ModelArenaService {

    @Override
    public List<ModelConfig> list() {
        return super.list();
    }

    @Override
    public ModelConfig getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(ModelConfig entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(ModelConfig entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
