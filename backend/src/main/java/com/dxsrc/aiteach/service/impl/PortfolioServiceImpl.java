package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Portfolio;
import com.dxsrc.aiteach.mapper.PortfolioMapper;
import com.dxsrc.aiteach.service.PortfolioService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class PortfolioServiceImpl extends ServiceImpl<PortfolioMapper, Portfolio> implements PortfolioService {

    @Override
    public List<Portfolio> list() {
        return super.list();
    }

    @Override
    public Portfolio getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Portfolio entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Portfolio entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
