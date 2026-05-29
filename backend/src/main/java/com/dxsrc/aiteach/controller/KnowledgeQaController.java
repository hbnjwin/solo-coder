package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.KnowledgeBase;
import com.dxsrc.aiteach.service.KnowledgeQaService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/knowledge-qa")
@RequiredArgsConstructor
public class KnowledgeQaController {

    private final KnowledgeQaService knowledgeQaService;

    @GetMapping("/list")
    public List<KnowledgeBase> list() {
        return knowledgeQaService.list();
    }

    @GetMapping("/{id}")
    public KnowledgeBase getById(@PathVariable Long id) {
        return knowledgeQaService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody KnowledgeBase entity) {
        return knowledgeQaService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody KnowledgeBase entity) {
        return knowledgeQaService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return knowledgeQaService.removeById(id);
    }
}
