package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.DialogueSession;
import com.dxsrc.aiteach.service.VoiceScoringService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/voice-scoring")
@RequiredArgsConstructor
public class VoiceScoringController {

    private final VoiceScoringService voiceScoringService;

    @GetMapping("/list")
    public List<DialogueSession> list() {
        return voiceScoringService.list();
    }

    @GetMapping("/{id}")
    public DialogueSession getById(@PathVariable Long id) {
        return voiceScoringService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody DialogueSession entity) {
        return voiceScoringService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody DialogueSession entity) {
        return voiceScoringService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return voiceScoringService.removeById(id);
    }
}
