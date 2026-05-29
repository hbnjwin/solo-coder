package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Video;
import java.util.List;

public interface VideoSubtitleService {

    List<Video> list();

    Video getById(Long id);

    boolean save(Video entity);

    boolean update(Video entity);

    boolean removeById(Long id);
}
