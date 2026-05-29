package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.DialogueSession;
import java.util.List;

public interface VoiceScoringService {

    List<DialogueSession> list();

    DialogueSession getById(Long id);

    boolean save(DialogueSession entity);

    boolean update(DialogueSession entity);

    boolean removeById(Long id);
}
