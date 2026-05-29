package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.CalendarEvent;
import com.dxsrc.aiteach.service.TeachingCalendarService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/teaching-calendar")
@RequiredArgsConstructor
public class TeachingCalendarController {

    private final TeachingCalendarService teachingCalendarService;

    @GetMapping("/list")
    public List<CalendarEvent> list() {
        return teachingCalendarService.list();
    }

    @GetMapping("/{id}")
    public CalendarEvent getById(@PathVariable Long id) {
        return teachingCalendarService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody CalendarEvent entity) {
        return teachingCalendarService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody CalendarEvent entity) {
        return teachingCalendarService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return teachingCalendarService.removeById(id);
    }
}
