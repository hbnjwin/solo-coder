package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Document;
import com.dxsrc.aiteach.service.WriteCollabService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/write-collab")
@RequiredArgsConstructor
public class WriteCollabController {

    private final WriteCollabService writeCollabService;

    @GetMapping("/list")
    public List<Document> list() {
        return writeCollabService.list();
    }

    @GetMapping("/{id}")
    public Document getById(@PathVariable Long id) {
        return writeCollabService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Document entity) {
        return writeCollabService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Document entity) {
        return writeCollabService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return writeCollabService.removeById(id);
    }
}
