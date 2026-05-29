package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Mindmap;
import com.dxsrc.aiteach.service.MindmapCollabService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/mindmap-collab")
@RequiredArgsConstructor
public class MindmapCollabController {

    private final MindmapCollabService mindmapCollabService;

    @GetMapping("/list")
    public List<Mindmap> list() {
        return mindmapCollabService.list();
    }

    @GetMapping("/{id}")
    public Mindmap getById(@PathVariable Long id) {
        return mindmapCollabService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Mindmap entity) {
        return mindmapCollabService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Mindmap entity) {
        return mindmapCollabService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return mindmapCollabService.removeById(id);
    }
}
