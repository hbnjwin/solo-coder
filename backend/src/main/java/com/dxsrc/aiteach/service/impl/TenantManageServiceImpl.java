package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Tenant;
import com.dxsrc.aiteach.mapper.TenantMapper;
import com.dxsrc.aiteach.service.TenantManageService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class TenantManageServiceImpl extends ServiceImpl<TenantMapper, Tenant> implements TenantManageService {

    @Override
    public List<Tenant> list() {
        return super.list();
    }

    @Override
    public Tenant getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Tenant entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Tenant entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
