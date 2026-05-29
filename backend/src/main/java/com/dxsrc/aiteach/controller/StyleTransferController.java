package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.StyleTemplate;
import com.dxsrc.aiteach.service.StyleTransferService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/style-transfer")
@RequiredArgsConstructor
public class StyleTransferController {

    private final StyleTransferService styleTransferService;

    @GetMapping("/list")
    public List<StyleTemplate> list() {
        return styleTransferService.list();
    }

    @GetMapping("/{id}")
    public StyleTemplate getById(@PathVariable Long id) {
        return styleTransferService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody StyleTemplate entity) {
        return styleTransferService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody StyleTemplate entity) {
        return styleTransferService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return styleTransferService.removeById(id);
    }
}
