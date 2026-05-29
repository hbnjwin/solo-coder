package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Homework;
import com.dxsrc.aiteach.service.HomeworkFeedbackService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/homework-feedback")
@RequiredArgsConstructor
public class HomeworkFeedbackController {

    private final HomeworkFeedbackService homeworkFeedbackService;

    @GetMapping("/list")
    public List<Homework> list() {
        return homeworkFeedbackService.list();
    }

    @GetMapping("/{id}")
    public Homework getById(@PathVariable Long id) {
        return homeworkFeedbackService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Homework entity) {
        return homeworkFeedbackService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Homework entity) {
        return homeworkFeedbackService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return homeworkFeedbackService.removeById(id);
    }
}
