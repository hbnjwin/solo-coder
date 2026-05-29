package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Resource;
import com.dxsrc.aiteach.service.ResourceShareService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/resource-share")
@RequiredArgsConstructor
public class ResourceShareController {

    private final ResourceShareService resourceShareService;

    @GetMapping("/list")
    public List<Resource> list() {
        return resourceShareService.list();
    }

    @GetMapping("/{id}")
    public Resource getById(@PathVariable Long id) {
        return resourceShareService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Resource entity) {
        return resourceShareService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Resource entity) {
        return resourceShareService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return resourceShareService.removeById(id);
    }
}
