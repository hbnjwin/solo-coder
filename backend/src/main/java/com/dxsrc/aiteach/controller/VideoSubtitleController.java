package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Video;
import com.dxsrc.aiteach.service.VideoSubtitleService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/video-subtitle")
@RequiredArgsConstructor
public class VideoSubtitleController {

    private final VideoSubtitleService videoSubtitleService;

    @GetMapping("/list")
    public List<Video> list() {
        return videoSubtitleService.list();
    }

    @GetMapping("/{id}")
    public Video getById(@PathVariable Long id) {
        return videoSubtitleService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Video entity) {
        return videoSubtitleService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Video entity) {
        return videoSubtitleService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return videoSubtitleService.removeById(id);
    }
}
