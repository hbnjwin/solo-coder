package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.MusicProject;
import java.util.List;

public interface MusicCollabService {

    List<MusicProject> list();

    MusicProject getById(Long id);

    boolean save(MusicProject entity);

    boolean update(MusicProject entity);

    boolean removeById(Long id);
}
