package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Resource;
import com.dxsrc.aiteach.mapper.ResourceMapper;
import com.dxsrc.aiteach.service.ResourceShareService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class ResourceShareServiceImpl extends ServiceImpl<ResourceMapper, Resource> implements ResourceShareService {

    @Override
    public List<Resource> list() {
        return super.list();
    }

    @Override
    public Resource getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Resource entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Resource entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
