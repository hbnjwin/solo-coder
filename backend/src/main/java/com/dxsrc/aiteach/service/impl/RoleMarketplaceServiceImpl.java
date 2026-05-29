package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.AiRole;
import com.dxsrc.aiteach.mapper.AiRoleMapper;
import com.dxsrc.aiteach.service.RoleMarketplaceService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class RoleMarketplaceServiceImpl extends ServiceImpl<AiRoleMapper, AiRole> implements RoleMarketplaceService {

    @Override
    public List<AiRole> list() {
        return super.list();
    }

    @Override
    public AiRole getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(AiRole entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(AiRole entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
