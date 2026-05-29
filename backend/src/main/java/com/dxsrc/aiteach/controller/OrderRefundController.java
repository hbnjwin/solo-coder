package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Order;
import com.dxsrc.aiteach.service.OrderRefundService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/order-refund")
@RequiredArgsConstructor
public class OrderRefundController {

    private final OrderRefundService orderRefundService;

    @GetMapping("/list")
    public List<Order> list() {
        return orderRefundService.list();
    }

    @GetMapping("/{id}")
    public Order getById(@PathVariable Long id) {
        return orderRefundService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Order entity) {
        return orderRefundService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Order entity) {
        return orderRefundService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return orderRefundService.removeById(id);
    }
}
