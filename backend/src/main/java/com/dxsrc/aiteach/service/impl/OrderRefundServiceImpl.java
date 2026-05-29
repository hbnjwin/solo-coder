package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Order;
import com.dxsrc.aiteach.mapper.OrderMapper;
import com.dxsrc.aiteach.service.OrderRefundService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class OrderRefundServiceImpl extends ServiceImpl<OrderMapper, Order> implements OrderRefundService {

    @Override
    public List<Order> list() {
        return super.list();
    }

    @Override
    public Order getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Order entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Order entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
