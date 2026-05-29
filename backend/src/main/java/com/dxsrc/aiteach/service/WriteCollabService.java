package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Document;
import java.util.List;

public interface WriteCollabService {

    List<Document> list();

    Document getById(Long id);

    boolean save(Document entity);

    boolean update(Document entity);

    boolean removeById(Long id);
}
