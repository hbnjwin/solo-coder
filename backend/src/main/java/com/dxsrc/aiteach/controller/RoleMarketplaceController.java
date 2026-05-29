package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.AiRole;
import com.dxsrc.aiteach.service.RoleMarketplaceService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/role-marketplace")
@RequiredArgsConstructor
public class RoleMarketplaceController {

    private final RoleMarketplaceService roleMarketplaceService;

    @GetMapping("/list")
    public List<AiRole> list() {
        return roleMarketplaceService.list();
    }

    @GetMapping("/{id}")
    public AiRole getById(@PathVariable Long id) {
        return roleMarketplaceService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody AiRole entity) {
        return roleMarketplaceService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody AiRole entity) {
        return roleMarketplaceService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return roleMarketplaceService.removeById(id);
    }
}
