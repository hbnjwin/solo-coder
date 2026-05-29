package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Timetable;
import com.dxsrc.aiteach.service.TimetableService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/timetable")
@RequiredArgsConstructor
public class TimetableController {

    private final TimetableService timetableService;

    @GetMapping("/list")
    public List<Timetable> list() {
        return timetableService.list();
    }

    @GetMapping("/{id}")
    public Timetable getById(@PathVariable Long id) {
        return timetableService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Timetable entity) {
        return timetableService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Timetable entity) {
        return timetableService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return timetableService.removeById(id);
    }
}
