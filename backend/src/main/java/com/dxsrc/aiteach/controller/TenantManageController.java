package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Tenant;
import com.dxsrc.aiteach.service.TenantManageService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/tenant-manage")
@RequiredArgsConstructor
public class TenantManageController {

    private final TenantManageService tenantManageService;

    @GetMapping("/list")
    public List<Tenant> list() {
        return tenantManageService.list();
    }

    @GetMapping("/{id}")
    public Tenant getById(@PathVariable Long id) {
        return tenantManageService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Tenant entity) {
        return tenantManageService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Tenant entity) {
        return tenantManageService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return tenantManageService.removeById(id);
    }
}
