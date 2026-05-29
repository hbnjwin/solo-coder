package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Competition;
import com.dxsrc.aiteach.mapper.CompetitionMapper;
import com.dxsrc.aiteach.service.CompetitionAuditService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class CompetitionAuditServiceImpl extends ServiceImpl<CompetitionMapper, Competition> implements CompetitionAuditService {

    @Override
    public List<Competition> list() {
        return super.list();
    }

    @Override
    public Competition getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Competition entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Competition entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
