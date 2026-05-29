package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.ModelConfig;
import com.dxsrc.aiteach.service.ModelArenaService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/model-arena")
@RequiredArgsConstructor
public class ModelArenaController {

    private final ModelArenaService modelArenaService;

    @GetMapping("/list")
    public List<ModelConfig> list() {
        return modelArenaService.list();
    }

    @GetMapping("/{id}")
    public ModelConfig getById(@PathVariable Long id) {
        return modelArenaService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody ModelConfig entity) {
        return modelArenaService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody ModelConfig entity) {
        return modelArenaService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return modelArenaService.removeById(id);
    }
}
