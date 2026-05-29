package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Classroom;
import java.util.List;

public interface LiveQuizService {

    List<Classroom> list();

    Classroom getById(Long id);

    boolean save(Classroom entity);

    boolean update(Classroom entity);

    boolean removeById(Long id);
}
