package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Exam;
import java.util.List;

public interface CorrectionAnalysisService {

    List<Exam> list();

    Exam getById(Long id);

    boolean save(Exam entity);

    boolean update(Exam entity);

    boolean removeById(Long id);
}
