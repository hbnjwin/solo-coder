package com.linkyoyo.tobacco.repository;

import com.cosium.spring.data.jpa.entity.graph.repository.EntityGraphJpaRepository;
import com.cosium.spring.data.jpa.entity.graph.repository.EntityGraphJpaSpecificationExecutor;
import com.linkyoyo.tobacco.entity.StandardDoc;

public interface StandardDocRepository extends EntityGraphJpaRepository<StandardDoc, Integer>, EntityGraphJpaSpecificationExecutor<StandardDoc> {
}
