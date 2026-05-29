package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Classroom;
import com.dxsrc.aiteach.service.LiveQuizService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/live-quiz")
@RequiredArgsConstructor
public class LiveQuizController {

    private final LiveQuizService liveQuizService;

    @GetMapping("/list")
    public List<Classroom> list() {
        return liveQuizService.list();
    }

    @GetMapping("/{id}")
    public Classroom getById(@PathVariable Long id) {
        return liveQuizService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Classroom entity) {
        return liveQuizService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Classroom entity) {
        return liveQuizService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return liveQuizService.removeById(id);
    }
}
