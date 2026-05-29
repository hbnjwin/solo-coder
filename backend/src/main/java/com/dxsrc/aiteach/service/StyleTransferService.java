package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.StyleTemplate;
import java.util.List;

public interface StyleTransferService {

    List<StyleTemplate> list();

    StyleTemplate getById(Long id);

    boolean save(StyleTemplate entity);

    boolean update(StyleTemplate entity);

    boolean removeById(Long id);
}
