package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Exam;
import com.dxsrc.aiteach.service.CorrectionAnalysisService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/correction-analysis")
@RequiredArgsConstructor
public class CorrectionAnalysisController {

    private final CorrectionAnalysisService correctionAnalysisService;

    @GetMapping("/list")
    public List<Exam> list() {
        return correctionAnalysisService.list();
    }

    @GetMapping("/{id}")
    public Exam getById(@PathVariable Long id) {
        return correctionAnalysisService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Exam entity) {
        return correctionAnalysisService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Exam entity) {
        return correctionAnalysisService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return correctionAnalysisService.removeById(id);
    }
}
