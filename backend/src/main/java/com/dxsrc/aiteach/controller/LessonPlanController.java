package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Subject;
import com.dxsrc.aiteach.service.LessonPlanService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/lesson-plan")
@RequiredArgsConstructor
public class LessonPlanController {

    private final LessonPlanService lessonPlanService;

    @GetMapping("/list")
    public List<Subject> list() {
        return lessonPlanService.list();
    }

    @GetMapping("/{id}")
    public Subject getById(@PathVariable Long id) {
        return lessonPlanService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Subject entity) {
        return lessonPlanService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Subject entity) {
        return lessonPlanService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return lessonPlanService.removeById(id);
    }
}
