package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.TranslationProject;
import com.dxsrc.aiteach.service.TranslationService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/translation")
@RequiredArgsConstructor
public class TranslationController {

    private final TranslationService translationService;

    @GetMapping("/list")
    public List<TranslationProject> list() {
        return translationService.list();
    }

    @GetMapping("/{id}")
    public TranslationProject getById(@PathVariable Long id) {
        return translationService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody TranslationProject entity) {
        return translationService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody TranslationProject entity) {
        return translationService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return translationService.removeById(id);
    }
}
