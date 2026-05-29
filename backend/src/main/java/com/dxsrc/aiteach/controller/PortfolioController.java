package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Portfolio;
import com.dxsrc.aiteach.service.PortfolioService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/portfolio")
@RequiredArgsConstructor
public class PortfolioController {

    private final PortfolioService portfolioService;

    @GetMapping("/list")
    public List<Portfolio> list() {
        return portfolioService.list();
    }

    @GetMapping("/{id}")
    public Portfolio getById(@PathVariable Long id) {
        return portfolioService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Portfolio entity) {
        return portfolioService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Portfolio entity) {
        return portfolioService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return portfolioService.removeById(id);
    }
}
