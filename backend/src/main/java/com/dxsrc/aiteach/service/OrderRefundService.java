package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Order;
import java.util.List;

public interface OrderRefundService {

    List<Order> list();

    Order getById(Long id);

    boolean save(Order entity);

    boolean update(Order entity);

    boolean removeById(Long id);
}
