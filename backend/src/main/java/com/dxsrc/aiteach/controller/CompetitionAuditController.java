package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Competition;
import com.dxsrc.aiteach.service.CompetitionAuditService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/competition-audit")
@RequiredArgsConstructor
public class CompetitionAuditController {

    private final CompetitionAuditService competitionAuditService;

    @GetMapping("/list")
    public List<Competition> list() {
        return competitionAuditService.list();
    }

    @GetMapping("/{id}")
    public Competition getById(@PathVariable Long id) {
        return competitionAuditService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Competition entity) {
        return competitionAuditService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Competition entity) {
        return competitionAuditService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return competitionAuditService.removeById(id);
    }
}
