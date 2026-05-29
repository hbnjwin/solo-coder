package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.KnowledgeSource;
import com.dxsrc.aiteach.service.KnowledgeSyncService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/knowledge-sync")
@RequiredArgsConstructor
public class KnowledgeSyncController {

    private final KnowledgeSyncService knowledgeSyncService;

    @GetMapping("/list")
    public List<KnowledgeSource> list() {
        return knowledgeSyncService.list();
    }

    @GetMapping("/{id}")
    public KnowledgeSource getById(@PathVariable Long id) {
        return knowledgeSyncService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody KnowledgeSource entity) {
        return knowledgeSyncService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody KnowledgeSource entity) {
        return knowledgeSyncService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return knowledgeSyncService.removeById(id);
    }
}
