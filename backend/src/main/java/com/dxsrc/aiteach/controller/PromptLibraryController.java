package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.PromptTemplate;
import com.dxsrc.aiteach.service.PromptLibraryService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/prompt-library")
@RequiredArgsConstructor
public class PromptLibraryController {

    private final PromptLibraryService promptLibraryService;

    @GetMapping("/list")
    public List<PromptTemplate> list() {
        return promptLibraryService.list();
    }

    @GetMapping("/{id}")
    public PromptTemplate getById(@PathVariable Long id) {
        return promptLibraryService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody PromptTemplate entity) {
        return promptLibraryService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody PromptTemplate entity) {
        return promptLibraryService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return promptLibraryService.removeById(id);
    }
}
