package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.MusicProject;
import com.dxsrc.aiteach.service.MusicCollabService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/music-collab")
@RequiredArgsConstructor
public class MusicCollabController {

    private final MusicCollabService musicCollabService;

    @GetMapping("/list")
    public List<MusicProject> list() {
        return musicCollabService.list();
    }

    @GetMapping("/{id}")
    public MusicProject getById(@PathVariable Long id) {
        return musicCollabService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody MusicProject entity) {
        return musicCollabService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody MusicProject entity) {
        return musicCollabService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return musicCollabService.removeById(id);
    }
}
